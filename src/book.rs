use crate::config::{Config, ItemLevel};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Book {
    pub config: Config,
    pub items: Vec<BookItem>,
}

#[derive(Debug, Clone)]
pub struct BookItem {
    pub title: String,
    pub level: ItemLevel,
    pub page: Option<PageInfo>,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    pub source_path: PathBuf,
    pub output_filename: String,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub title: String,
    pub id: String,
}

impl Book {
    pub fn from_config(config: Config, base_dir: &Path) -> Result<Self> {
        let mut items = Vec::new();

        for item_config in &config.items {
            let page = match &item_config.path {
                None => None,
                Some(path) => {
                    // Has path = create PageInfo
                    let source_path = base_dir.join(&config.build.src_dir).join(path);

                    // Validate that source file exists
                    if !source_path.exists() {
                        anyhow::bail!(
                            "Source file not found: {} (looking for: {})",
                            path,
                            source_path.display()
                        );
                    }

                    // Convert path to output filename (e.g., intro.md -> intro.html)
                    let output_filename = Self::source_to_html_filename(path)?;

                    // Extract H2 sections from markdown
                    let sections = Self::extract_sections(&source_path)?;

                    Some(PageInfo {
                        source_path,
                        output_filename,
                        sections,
                    })
                }
            };

            items.push(BookItem {
                title: item_config.title.clone(),
                level: item_config.level.clone(),
                page,
            });
        }

        Ok(Self { config, items })
    }

    fn source_to_html_filename(source_path: &str) -> Result<String> {
        // Validate that the source path ends with .md
        if !source_path.ends_with(".md") {
            anyhow::bail!("Source file must have .md extension: {}", source_path);
        }

        // Replace .md with .html while preserving directory structure
        let html_filename = source_path.replace(".md", ".html");
        Ok(html_filename)
    }

    fn extract_sections(markdown_path: &Path) -> Result<Vec<Section>> {
        let content =
            std::fs::read_to_string(markdown_path).context("Failed to read markdown file")?;

        let mut sections = Vec::new();

        for line in content.lines() {
            // Check if line is an H2 heading (## Title)
            if let Some(title) = line.strip_prefix("## ") {
                let title = title.trim().to_string();
                // Generate ID matching unidoc's format: "2-Title"
                // unidoc percent-encodes all except alphanumeric, '-', and '_'
                let id = format!("2-{}", Self::percent_encode(&title));
                sections.push(Section { title, id });
            }
        }

        Ok(sections)
    }

    fn percent_encode(input: &str) -> String {
        // Match unidoc's encoding: NON_ALPHANUMERIC minus '-' and '_'
        // This means: encode everything except [a-zA-Z0-9-_]
        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c.to_string()
                } else {
                    // Percent encode the character
                    c.to_string()
                        .bytes()
                        .map(|b| format!("%{:02X}", b))
                        .collect::<String>()
                }
            })
            .collect()
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
    use crate::config::{BookConfig, BuildConfig, Config, ItemConfig, ItemLevel, TocConfig};

    fn create_test_config() -> Config {
        Config {
            book: BookConfig {
                title: "Test Book".to_string(),
                description: None,
                authors: vec![],
                language: "en".to_string(),
                theme: "light".to_string(),
            },
            build: BuildConfig {
                src_dir: PathBuf::from("src"),
                output_dir: PathBuf::from("docs"),
                base_path: String::new(),
            },
            toc: TocConfig {
                show_sections: "current".to_string(),
            },
            items: vec![
                ItemConfig {
                    title: "Page 1".to_string(),
                    level: ItemLevel::Page,
                    path: Some("page1.md".to_string()),
                },
                ItemConfig {
                    title: "Page 2".to_string(),
                    level: ItemLevel::Page,
                    path: Some("page2.md".to_string()),
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
        // Test subdirectory paths
        assert_eq!(Book::source_to_html_filename("a/b.md").unwrap(), "a/b.html");
        assert_eq!(
            Book::source_to_html_filename("foo/bar/baz.md").unwrap(),
            "foo/bar/baz.html"
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

        assert_eq!(book.items.len(), 2);

        assert_eq!(book.items[0].title, "Page 1");
        assert!(book.items[0].page.is_some());
        assert_eq!(
            book.items[0].page.as_ref().unwrap().output_filename,
            "page1.html"
        );

        assert_eq!(book.items[1].title, "Page 2");
        assert!(book.items[1].page.is_some());
        assert_eq!(
            book.items[1].page.as_ref().unwrap().output_filename,
            "page2.html"
        );

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
            source_path: PathBuf::from("test.md"),
            output_filename: "test.html".to_string(),
            sections: vec![],
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

        assert_eq!(output_dir, temp_dir.join("docs"));

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
