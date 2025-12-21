use crate::book::Book;
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct SearchIndex {
    pages: Vec<SearchEntry>,
}

#[derive(Debug, Serialize)]
pub struct SearchEntry {
    title: String,
    url: String,
    content: String,
}

pub struct SearchIndexGenerator;

impl SearchIndexGenerator {
    pub fn generate(book: &Book, output_dir: &Path) -> Result<()> {
        let mut entries = Vec::new();

        for page in &book.pages {
            let content = Self::extract_text_from_markdown(&page.source_path)?;
            entries.push(SearchEntry {
                title: page.title.clone(),
                url: page.output_filename.clone(),
                content,
            });
        }

        let index = SearchIndex { pages: entries };
        let json =
            serde_json::to_string_pretty(&index).context("Failed to serialize search index")?;

        let index_path = output_dir.join("search-index.json");
        fs::write(&index_path, json).context("Failed to write search index")?;

        Ok(())
    }

    fn extract_text_from_markdown(path: &Path) -> Result<String> {
        let content = fs::read_to_string(path).context("Failed to read markdown file")?;

        // Simple text extraction: remove markdown syntax
        let text = Self::strip_markdown(&content);

        // Normalize whitespace
        let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");

        Ok(normalized)
    }

    fn strip_markdown(text: &str) -> String {
        let mut result = String::new();
        let mut in_code_block = false;

        for line in text.lines() {
            // Toggle code block state
            if line.trim_start().starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            // Include code blocks in search
            if in_code_block {
                result.push_str(line);
                result.push(' ');
                continue;
            }

            // Remove common markdown syntax
            let cleaned = line
                .trim_start_matches('#') // Headers
                .trim_start_matches('-') // List items
                .trim_start_matches('*') // List items
                .trim_start_matches('>') // Blockquotes
                .replace("**", "") // Bold
                .replace("__", "") // Bold
                .replace(['*', '_', '`'], ""); // Italic and inline code

            result.push_str(&cleaned);
            result.push(' ');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_markdown() {
        let input = "# Header\n**bold** *italic*";
        let output = SearchIndexGenerator::strip_markdown(input);
        assert!(!output.contains('#'));
        assert!(!output.contains('*'));
    }
}
