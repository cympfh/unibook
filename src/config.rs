use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub book: BookConfig,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub toc: TocConfig,
    pub pages: Vec<PageConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BookConfig {
    pub title: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub authors: Vec<String>,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "light".to_string()
}

fn default_language() -> String {
    "ja".to_string()
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_src_dir")]
    pub src_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    #[serde(default = "default_base_path")]
    pub base_path: String,
}

#[derive(Debug, Deserialize)]
pub struct TocConfig {
    /// When to show H2 sections in TOC
    /// - "always": Show sections for all pages
    /// - "current": Show sections only for current page (default)
    /// - "never": Never show sections
    #[serde(default = "default_show_sections")]
    pub show_sections: String,
    /// Fold level for parts
    /// Parts with level >= foldlevel will be folded by default
    /// 0 = no folding (default), 1 = fold level 1+, 2 = fold level 2+, 3 = fold level 3+
    #[serde(default = "default_foldlevel")]
    pub foldlevel: u8,
}

fn default_show_sections() -> String {
    "current".to_string()
}

fn default_foldlevel() -> u8 {
    0
}

#[derive(Debug, Deserialize, Clone)]
pub struct PageConfig {
    pub title: String,
    /// Path to the markdown file. If None, this is a part (separator/heading only)
    pub path: Option<String>,
    /// Level for parts (1=大見出し/Part, 2=中見出し/Chapter, 3=小見出し/Section)
    /// Only used when path is None. Defaults to 1 if not specified.
    #[serde(default = "default_part_level")]
    pub level: u8,
    /// Child pages under this part (only valid when path is None)
    /// - None: auto-group following pages (default)
    /// - Some([]): no children
    /// - Some([...]): explicit children
    #[serde(default)]
    pub items: Option<Vec<PageItem>>,
}

/// A page item that can be nested under a part
#[derive(Debug, Deserialize, Clone)]
pub struct PageItem {
    pub title: String,
    pub path: String,
}

fn default_part_level() -> u8 {
    1
}

fn default_src_dir() -> PathBuf {
    PathBuf::from("src")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("docs")
}

fn default_base_path() -> String {
    String::new()
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            src_dir: default_src_dir(),
            output_dir: default_output_dir(),
            base_path: default_base_path(),
        }
    }
}

impl Default for TocConfig {
    fn default() -> Self {
        Self {
            show_sections: default_show_sections(),
            foldlevel: default_foldlevel(),
        }
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path.display()))?;
        let config: Config = toml::from_str(&contents).context("Failed to parse book.toml")?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.book.title.is_empty() {
            anyhow::bail!("Book title cannot be empty");
        }
        if self.pages.is_empty() {
            anyhow::bail!("No pages defined in book.toml");
        }
        // Check for duplicate titles
        let mut titles = std::collections::HashSet::new();
        for page in &self.pages {
            if !titles.insert(&page.title) {
                anyhow::bail!("Duplicate page title: {}", page.title);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_parse_valid() {
        let toml_content = r#"
[book]
title = "Test Book"
description = "A test"
authors = ["Alice", "Bob"]

[build]
src_dir = "source"
output_dir = "output"

[[pages]]
title = "Page 1"
path = "page1.md"

[[pages]]
title = "Page 2"
path = "page2.md"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.book.title, "Test Book");
        assert_eq!(config.book.description, Some("A test".to_string()));
        assert_eq!(config.book.authors, vec!["Alice", "Bob"]);
        assert_eq!(config.build.src_dir, PathBuf::from("source"));
        assert_eq!(config.build.output_dir, PathBuf::from("output"));
        assert_eq!(config.pages.len(), 2);
        assert_eq!(config.pages[0].title, "Page 1");
        assert_eq!(config.pages[0].path, Some("page1.md".to_string()));
    }

    #[test]
    fn test_config_defaults() {
        let toml_content = r#"
[book]
title = "Test Book"

[[pages]]
title = "Page 1"
path = "page1.md"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.build.src_dir, PathBuf::from("src"));
        assert_eq!(config.build.output_dir, PathBuf::from("docs"));
        assert_eq!(config.book.description, None);
        assert!(config.book.authors.is_empty());
    }

    #[test]
    fn test_config_validation_empty_title() {
        let toml_content = r#"
[book]
title = ""

[[pages]]
title = "Page 1"
path = "page1.md"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_no_pages() {
        let config = Config {
            book: BookConfig {
                title: "Test Book".to_string(),
                description: None,
                authors: vec![],
                language: "en".to_string(),
                theme: "light".to_string(),
            },
            build: BuildConfig::default(),
            toc: TocConfig::default(),
            pages: vec![],
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_duplicate_titles() {
        let toml_content = r#"
[book]
title = "Test Book"

[[pages]]
title = "Page 1"
path = "page1.md"

[[pages]]
title = "Page 1"
path = "page2.md"
"#;
        let config: Config = toml::from_str(toml_content).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_book.toml");

        let toml_content = r#"
[book]
title = "Test Book"

[[pages]]
title = "Page 1"
path = "page1.md"
"#;

        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        let config = Config::from_file(&config_path).unwrap();
        assert_eq!(config.book.title, "Test Book");

        std::fs::remove_file(&config_path).ok();
    }
}
