use crate::config::Config;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Book {
    pub config: Config,
    pub pages: Vec<PageInfo>,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    pub title: String,
    pub source_path: PathBuf,
    pub output_filename: String,
}

impl Book {
    pub fn from_config(config: Config, base_dir: &Path) -> Result<Self> {
        let mut pages = Vec::new();

        for page_config in &config.pages {
            let source_path = base_dir.join(&config.build.src_dir).join(&page_config.path);

            // Validate that source file exists
            if !source_path.exists() {
                anyhow::bail!(
                    "Source file not found: {} (looking for: {})",
                    page_config.path,
                    source_path.display()
                );
            }

            // Convert path to output filename (e.g., intro.md -> intro.html)
            let output_filename = Self::source_to_html_filename(&page_config.path)?;

            pages.push(PageInfo {
                title: page_config.title.clone(),
                source_path,
                output_filename,
            });
        }

        Ok(Self { config, pages })
    }

    fn source_to_html_filename(source_path: &str) -> Result<String> {
        let path = Path::new(source_path);
        let filename = path
            .file_name()
            .context("Invalid source path")?
            .to_str()
            .context("Invalid UTF-8 in filename")?;

        if !filename.ends_with(".md") {
            anyhow::bail!("Source file must have .md extension: {}", filename);
        }

        let html_filename = filename.replace(".md", ".html");
        Ok(html_filename)
    }

    pub fn output_dir(&self, base_dir: &Path) -> PathBuf {
        base_dir.join(&self.config.build.output_dir)
    }
}

impl PageInfo {
    pub fn slug(&self) -> String {
        self.output_filename.replace(".html", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{BookConfig, BuildConfig, Config, PageConfig};

    fn create_test_config() -> Config {
        Config {
            book: BookConfig {
                title: "Test Book".to_string(),
                description: None,
                authors: vec![],
            },
            build: BuildConfig {
                src_dir: PathBuf::from("src"),
                output_dir: PathBuf::from("doc"),
            },
            pages: vec![
                PageConfig {
                    title: "Page 1".to_string(),
                    path: "page1.md".to_string(),
                },
                PageConfig {
                    title: "Page 2".to_string(),
                    path: "page2.md".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_source_to_html_filename() {
        assert_eq!(
            Book::source_to_html_filename("intro.md").unwrap(),
            "intro.html"
        );
        assert_eq!(
            Book::source_to_html_filename("chapter1.md").unwrap(),
            "chapter1.html"
        );
    }

    #[test]
    fn test_source_to_html_filename_invalid() {
        assert!(Book::source_to_html_filename("intro.txt").is_err());
        assert!(Book::source_to_html_filename("noextension").is_err());
    }

    #[test]
    fn test_book_from_config() {
        let temp_dir = std::env::temp_dir().join("unibook-test-book");
        let src_dir = temp_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        // Create test files
        std::fs::write(src_dir.join("page1.md"), "# Page 1").unwrap();
        std::fs::write(src_dir.join("page2.md"), "# Page 2").unwrap();

        let config = create_test_config();
        let book = Book::from_config(config, &temp_dir).unwrap();

        assert_eq!(book.pages.len(), 2);
        assert_eq!(book.pages[0].title, "Page 1");
        assert_eq!(book.pages[0].output_filename, "page1.html");
        assert_eq!(book.pages[1].title, "Page 2");
        assert_eq!(book.pages[1].output_filename, "page2.html");

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_book_from_config_missing_file() {
        let temp_dir = std::env::temp_dir().join("unibook-test-book-missing");
        let config = create_test_config();

        // Don't create the files - should fail
        let result = Book::from_config(config, &temp_dir);
        assert!(result.is_err());

        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_page_info_slug() {
        let page = PageInfo {
            title: "Test Page".to_string(),
            source_path: PathBuf::from("test.md"),
            output_filename: "test.html".to_string(),
        };
        assert_eq!(page.slug(), "test");
    }

    #[test]
    fn test_output_dir() {
        let config = create_test_config();
        let temp_dir = std::env::temp_dir().join("unibook-test");
        let src_dir = temp_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("page1.md"), "# Page 1").unwrap();
        std::fs::write(src_dir.join("page2.md"), "# Page 2").unwrap();

        let book = Book::from_config(config, &temp_dir).unwrap();
        let output_dir = book.output_dir(&temp_dir);

        assert_eq!(output_dir, temp_dir.join("doc"));

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
