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

        // Generate all assets
        self.generate_assets()?;

        // Build each page
        let toc_gen = TocGenerator::new(
            self.book.config.book.title.clone(),
            self.book.config.toc.show_sections.clone(),
            self.book.config.build.base_path.clone(),
            self.book.config.toc.foldlevel,
        );

        for item in &self.book.items {
            match item {
                crate::book::BookItem::Part { children, .. } => {
                    // Build all child pages under this part
                    for page in children {
                        println!(
                            "Building: {} -> {}",
                            page.source_path.display(),
                            page.output_filename
                        );

                        // Generate TOC with current page highlighted
                        let toc_html = toc_gen
                            .generate_toc_html(&self.book.items, Some(&page.output_filename));
                        // Replace path separators in slug to avoid creating subdirectories in temp_dir
                        let slug = page.slug().replace(['/', '\\'], "_");
                        let toc_path = self.temp_dir.join(format!("toc-{}.html", slug));
                        fs::write(&toc_path, toc_html).context("Failed to write TOC file")?;

                        // Build the page
                        let output_file = output_dir.join(&page.output_filename);

                        // Create parent directories if they don't exist
                        if let Some(parent) = output_file.parent() {
                            fs::create_dir_all(parent)
                                .context("Failed to create output subdirectories")?;
                        }

                        self.build_page(page, &toc_path, &output_file)?;
                    }
                }
                crate::book::BookItem::Page(page) => {
                    println!(
                        "Building: {} -> {}",
                        page.source_path.display(),
                        page.output_filename
                    );

                    // Generate TOC with current page highlighted
                    let toc_html =
                        toc_gen.generate_toc_html(&self.book.items, Some(&page.output_filename));
                    // Replace path separators in slug to avoid creating subdirectories in temp_dir
                    let slug = page.slug().replace(['/', '\\'], "_");
                    let toc_path = self.temp_dir.join(format!("toc-{}.html", slug));
                    fs::write(&toc_path, toc_html).context("Failed to write TOC file")?;

                    // Build the page
                    let output_file = output_dir.join(&page.output_filename);

                    // Create parent directories if they don't exist
                    if let Some(parent) = output_file.parent() {
                        fs::create_dir_all(parent)
                            .context("Failed to create output subdirectories")?;
                    }

                    self.build_page(page, &toc_path, &output_file)?;
                }
            }
        }

        // Generate search index
        println!("Generating search index...");
        crate::search::SearchIndexGenerator::generate(&self.book, &output_dir)
            .context("Failed to generate search index")?;

        // Generate index.html that redirects to first page
        let first_page = self.book.items.iter().find_map(|item| match item {
            crate::book::BookItem::Part { children, .. } => children.first(),
            crate::book::BookItem::Page(page) => Some(page),
        });

        if let Some(first_page) = first_page {
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

    fn add_lang_attribute(&self, html_file: &Path) -> Result<()> {
        let content = fs::read_to_string(html_file).context("Failed to read HTML file")?;

        let modified = content.replace(
            "<html>",
            &format!("<html lang=\"{}\">", self.book.config.book.language),
        );

        fs::write(html_file, modified).context("Failed to write HTML file with lang attribute")?;

        Ok(())
    }

    /// Build only pages that have been modified
    /// If changed_file is None or book.toml, rebuild everything
    /// Otherwise, only rebuild the specific changed file
    pub fn build_incremental(&self, changed_file: Option<&Path>) -> Result<()> {
        // If book.toml changed or no specific file, do full rebuild
        if changed_file.is_none()
            || changed_file
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                == Some("book.toml")
        {
            return self.build();
        }

        let changed_file = changed_file.unwrap();
        let output_dir = self.book.output_dir(&self.base_dir);
        fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

        // Generate all assets (these are lightweight)
        self.generate_assets()?;

        // Normalize changed file path for comparison
        let changed_file_canonical = changed_file.canonicalize().ok();

        // Find the page that corresponds to this changed file
        let changed_page = self.book.items.iter().find_map(|item| match item {
            crate::book::BookItem::Page(page) => {
                let page_canonical = page.source_path.canonicalize().ok();
                if page_canonical.is_some() && page_canonical == changed_file_canonical {
                    Some(page)
                } else {
                    None
                }
            }
            crate::book::BookItem::Part { children, .. } => children.iter().find(|page| {
                let page_canonical = page.source_path.canonicalize().ok();
                page_canonical.is_some() && page_canonical == changed_file_canonical
            }),
        });

        if let Some(page) = changed_page {
            println!(
                "Building: {} -> {}",
                page.source_path.display(),
                page.output_filename
            );

            // Generate TOC with current page highlighted
            let toc_gen = TocGenerator::new(
                self.book.config.book.title.clone(),
                self.book.config.toc.show_sections.clone(),
                self.book.config.build.base_path.clone(),
                self.book.config.toc.foldlevel,
            );
            let toc_html = toc_gen.generate_toc_html(&self.book.items, Some(&page.output_filename));
            let slug = page.slug().replace(['/', '\\'], "_");
            let toc_path = self.temp_dir.join(format!("toc-{}.html", slug));
            fs::write(&toc_path, toc_html).context("Failed to write TOC file")?;

            // Build the page
            let output_file = output_dir.join(&page.output_filename);
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent).context("Failed to create output subdirectories")?;
            }

            self.build_page(page, &toc_path, &output_file)?;

            // Regenerate search index (this is relatively fast)
            println!("Updating search index...");
            crate::search::SearchIndexGenerator::generate(&self.book, &output_dir)
                .context("Failed to generate search index")?;

            println!("\nIncremental build complete!");
        } else {
            // File is not in pages list, might be a new file or book.toml changed
            // Do a full rebuild
            println!("File not in current page list, doing full rebuild...");
            return self.build();
        }

        Ok(())
    }

    fn generate_assets(&self) -> Result<()> {
        let css_path = self.temp_dir.join("style.html");
        let css = TocGenerator::generate_css();
        fs::write(&css_path, css).context("Failed to write CSS file")?;

        let wrapper_end_path = self.temp_dir.join("wrapper-end.html");
        fs::write(&wrapper_end_path, TocGenerator::generate_wrapper_end())
            .context("Failed to write wrapper end file")?;

        let theme_css_path = self.temp_dir.join("theme-style.html");
        let theme_css = format!(
            "<style>{}</style>",
            crate::search_assets::SearchAssets::theme_css()
        );
        fs::write(&theme_css_path, theme_css).context("Failed to write theme CSS")?;

        let theme_switcher_css_path = self.temp_dir.join("theme-switcher-style.html");
        let theme_switcher_css = format!(
            "<style>{}</style>",
            crate::search_assets::SearchAssets::theme_switcher_css()
        );
        fs::write(&theme_switcher_css_path, theme_switcher_css)
            .context("Failed to write theme switcher CSS")?;

        let theme_meta_path = self.temp_dir.join("theme-meta.html");
        let theme_meta = format!(
            r#"<meta name="unibook-theme" content="{}">"#,
            self.book.config.book.theme
        );
        fs::write(&theme_meta_path, theme_meta).context("Failed to write theme meta")?;

        let theme_switcher_html_path = self.temp_dir.join("theme-switcher.html");
        fs::write(
            &theme_switcher_html_path,
            crate::search_assets::SearchAssets::theme_switcher_html(),
        )
        .context("Failed to write theme switcher HTML")?;

        let theme_switcher_js_path = self.temp_dir.join("theme-switcher-script.html");
        let theme_switcher_js = format!(
            "<script>{}</script>",
            crate::search_assets::SearchAssets::theme_switcher_js()
        );
        fs::write(&theme_switcher_js_path, theme_switcher_js)
            .context("Failed to write theme switcher JS")?;

        let toc_toggle_css_path = self.temp_dir.join("toc-toggle-style.html");
        let toc_toggle_css = format!(
            "<style>{}</style>",
            crate::search_assets::SearchAssets::toc_toggle_css()
        );
        fs::write(&toc_toggle_css_path, toc_toggle_css)
            .context("Failed to write TOC toggle CSS")?;

        let toc_toggle_html_path = self.temp_dir.join("toc-toggle.html");
        fs::write(
            &toc_toggle_html_path,
            crate::search_assets::SearchAssets::toc_toggle_html(),
        )
        .context("Failed to write TOC toggle HTML")?;

        let toc_toggle_js_path = self.temp_dir.join("toc-toggle-script.html");
        let toc_toggle_js = format!(
            "<script>{}</script>",
            crate::search_assets::SearchAssets::toc_toggle_js()
        );
        fs::write(&toc_toggle_js_path, toc_toggle_js).context("Failed to write TOC toggle JS")?;

        let code_copy_css_path = self.temp_dir.join("code-copy-style.html");
        let code_copy_css = format!(
            "<style>{}</style>",
            crate::search_assets::SearchAssets::code_copy_css()
        );
        fs::write(&code_copy_css_path, code_copy_css).context("Failed to write code copy CSS")?;

        let code_copy_js_path = self.temp_dir.join("code-copy-script.html");
        let code_copy_js = format!(
            "<script>{}</script>",
            crate::search_assets::SearchAssets::code_copy_js()
        );
        fs::write(&code_copy_js_path, code_copy_js).context("Failed to write code copy JS")?;

        let page_controls_start_path = self.temp_dir.join("page-controls-start.html");
        fs::write(
            &page_controls_start_path,
            crate::search_assets::SearchAssets::page_controls_start(),
        )
        .context("Failed to write page controls start")?;

        let page_controls_end_path = self.temp_dir.join("page-controls-end.html");
        fs::write(
            &page_controls_end_path,
            crate::search_assets::SearchAssets::page_controls_end(),
        )
        .context("Failed to write page controls end")?;

        let search_html_path = self.temp_dir.join("search.html");
        fs::write(
            &search_html_path,
            crate::search_assets::SearchAssets::html(),
        )
        .context("Failed to write search HTML")?;

        let search_css_path = self.temp_dir.join("search-style.html");
        let search_css = format!(
            "<style>{}</style>",
            crate::search_assets::SearchAssets::css()
        );
        fs::write(&search_css_path, search_css).context("Failed to write search CSS")?;

        let search_js_path = self.temp_dir.join("search-script.html");
        let search_js = format!(
            "<script>{}</script>",
            crate::search_assets::SearchAssets::javascript()
        );
        fs::write(&search_js_path, search_js).context("Failed to write search JS")?;

        let prism_retry_js_path = self.temp_dir.join("prism-retry-script.html");
        let prism_retry_js = format!(
            "<script>{}</script>",
            crate::search_assets::SearchAssets::prism_retry_js()
        );
        fs::write(&prism_retry_js_path, prism_retry_js)
            .context("Failed to write Prism retry JS")?;

        Ok(())
    }

    fn build_page(
        &self,
        page: &crate::book::PageInfo,
        toc_path: &Path,
        output_file: &Path,
    ) -> Result<()> {
        let theme_meta_path = self.temp_dir.join("theme-meta.html");
        let theme_css_path = self.temp_dir.join("theme-style.html");
        let theme_switcher_css_path = self.temp_dir.join("theme-switcher-style.html");
        let toc_toggle_css_path = self.temp_dir.join("toc-toggle-style.html");
        let code_copy_css_path = self.temp_dir.join("code-copy-style.html");
        let css_path = self.temp_dir.join("style.html");
        let search_css_path = self.temp_dir.join("search-style.html");
        let page_controls_start_path = self.temp_dir.join("page-controls-start.html");
        let toc_toggle_html_path = self.temp_dir.join("toc-toggle.html");
        let theme_switcher_html_path = self.temp_dir.join("theme-switcher.html");
        let page_controls_end_path = self.temp_dir.join("page-controls-end.html");
        let search_html_path = self.temp_dir.join("search.html");
        let theme_switcher_js_path = self.temp_dir.join("theme-switcher-script.html");
        let search_js_path = self.temp_dir.join("search-script.html");
        let toc_toggle_js_path = self.temp_dir.join("toc-toggle-script.html");
        let code_copy_js_path = self.temp_dir.join("code-copy-script.html");
        let prism_retry_js_path = self.temp_dir.join("prism-retry-script.html");
        let wrapper_end_path = self.temp_dir.join("wrapper-end.html");

        UnidocCommand::new()
            .standalone()
            .include_in_header(theme_meta_path)
            .include_in_header(theme_css_path)
            .include_in_header(theme_switcher_css_path)
            .include_in_header(toc_toggle_css_path)
            .include_in_header(code_copy_css_path)
            .include_in_header(css_path)
            .include_in_header(search_css_path)
            .include_before_body(toc_path.to_path_buf())
            .include_before_body(page_controls_start_path)
            .include_before_body(toc_toggle_html_path)
            .include_before_body(theme_switcher_html_path)
            .include_before_body(page_controls_end_path)
            .include_before_body(search_html_path)
            .include_after_body(theme_switcher_js_path)
            .include_after_body(search_js_path)
            .include_after_body(toc_toggle_js_path)
            .include_after_body(code_copy_js_path)
            .include_after_body(prism_retry_js_path)
            .include_after_body(wrapper_end_path)
            .output(output_file.to_path_buf())
            .execute(&page.source_path)
            .context(format!("Failed to build page: {}", page.title))?;

        self.add_lang_attribute(output_file)?;
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
