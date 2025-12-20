use crate::book::Book;
use crate::toc::TocGenerator;
use crate::unidoc::UnidocCommand;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Builder {
    book: Book,
    base_dir: PathBuf,
    temp_dir: PathBuf,
}

impl Builder {
    pub fn new(book: Book, base_dir: &Path) -> Result<Self> {
        let temp_dir = std::env::temp_dir().join("unibook-build");
        fs::create_dir_all(&temp_dir).context("Failed to create temporary directory")?;

        Ok(Self {
            book,
            base_dir: base_dir.to_path_buf(),
            temp_dir,
        })
    }

    pub fn build(&self) -> Result<()> {
        // Create output directory
        let output_dir = self.book.output_dir(&self.base_dir);
        fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

        // Generate CSS once
        let css_path = self.temp_dir.join("style.html");
        let css = TocGenerator::generate_css();
        fs::write(&css_path, css).context("Failed to write CSS file")?;

        // Generate wrapper end once
        let wrapper_end_path = self.temp_dir.join("wrapper-end.html");
        fs::write(&wrapper_end_path, TocGenerator::generate_wrapper_end())
            .context("Failed to write wrapper end file")?;

        // Build each page
        let toc_gen = TocGenerator::new(self.book.config.book.title.clone());

        for page in &self.book.pages {
            println!(
                "Building: {} -> {}",
                page.source_path.display(),
                page.output_filename
            );

            // Generate TOC with current page highlighted
            let toc_html = toc_gen.generate_toc_html(&self.book.pages, Some(&page.output_filename));
            let toc_path = self.temp_dir.join(format!("toc-{}.html", page.slug()));
            fs::write(&toc_path, toc_html).context("Failed to write TOC file")?;

            // Build unidoc command
            let output_file = output_dir.join(&page.output_filename);
            UnidocCommand::new()
                .standalone()
                .include_in_header(css_path.clone())
                .include_before_body(toc_path)
                .include_after_body(wrapper_end_path.clone())
                .output(output_file)
                .execute(&page.source_path)
                .context(format!("Failed to build page: {}", page.title))?;
        }

        // Generate index.html that redirects to first page
        if let Some(first_page) = self.book.pages.first() {
            let index_path = output_dir.join("index.html");
            let index_content = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="0; url={}">
    <title>Redirecting...</title>
</head>
<body>
    <p>Redirecting to <a href="{}">{}</a>...</p>
</body>
</html>"#,
                first_page.output_filename, first_page.output_filename, self.book.config.book.title
            );
            fs::write(&index_path, index_content).context("Failed to create index.html")?;
            println!("Created index.html");
        }

        println!("\nBuild complete! Output in: {}", output_dir.display());
        self.cleanup()?;
        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
        // Ignore errors during cleanup
        fs::remove_dir_all(&self.temp_dir).ok();
        Ok(())
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        // Ensure cleanup on drop
        let _ = self.cleanup();
    }
}
