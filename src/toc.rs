use crate::book::BookItem;
#[cfg(test)]
use crate::book::PageInfo;
use crate::config::ItemLevel;

pub struct TocGenerator {
    book_title: String,
    show_sections: String,
    base_path: String,
}

impl TocGenerator {
    pub fn new(book_title: String, show_sections: String, base_path: String) -> Self {
        // Normalize base_path:
        // - Add "/" at the beginning if not present
        // - Add "/" at the end if not present
        let normalized_base_path = if base_path.is_empty() {
            String::new()
        } else {
            let mut path = base_path;
            // Add leading "/" if missing
            if !path.starts_with('/') {
                path = format!("/{}", path);
            }
            // Add trailing "/" if missing
            if !path.ends_with('/') {
                path = format!("{}/", path);
            }
            // Remove the trailing "/" for storage (we'll add it in generate_toc_html)
            path.trim_end_matches('/').to_string()
        };

        Self {
            book_title,
            show_sections,
            base_path: normalized_base_path,
        }
    }

    pub fn generate_toc_html(&self, items: &[BookItem], current_page: Option<&str>) -> String {
        let mut html = String::new();
        html.push_str("<nav id=\"toc-sidebar\">\n");
        html.push_str(&format!(
            "  <div class=\"toc-header\"><h2>{}</h2></div>\n",
            html_escape(&self.book_title)
        ));
        html.push_str(
            "  <button id=\"search-button\" class=\"search-button\" title=\"Search (Ctrl+K)\">\n",
        );
        html.push_str(
            "    <svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" fill=\"currentColor\">\n",
        );
        html.push_str("      <path d=\"M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z\"/>\n");
        html.push_str("    </svg>\n");
        html.push_str("    Search\n");
        html.push_str("  </button>\n");
        html.push_str("  <ul class=\"toc-list\">\n");

        for item in items {
            // Determine CSS class based on item level
            let level_class = match item.level {
                ItemLevel::Part => "toc-part",
                ItemLevel::Chapter => "toc-chapter",
                ItemLevel::Section => "toc-section",
                ItemLevel::Subsection => "toc-subsection",
                ItemLevel::Page => "toc-page",
            };

            match &item.page {
                None => {
                    // No page content = render as heading/separator only
                    html.push_str(&format!(
                        "    <li class=\"{}\">{}</li>\n",
                        level_class,
                        html_escape(&item.title)
                    ));
                }
                Some(page) => {
                    // Has page content = render as link
                    let is_current = Some(page.output_filename.as_str()) == current_page;
                    let current_class = if is_current { " current" } else { "" };
                    html.push_str(&format!(
                        "    <li class=\"{}{}\">\n      <a href=\"{}/{}\">{}</a>\n",
                        level_class,
                        current_class,
                        self.base_path,
                        html_escape(&page.output_filename),
                        html_escape(&item.title)
                    ));

                    // Add sections based on show_sections setting
                    let should_show = match self.show_sections.as_str() {
                        "always" => true,
                        "current" => is_current,
                        "never" => false,
                        _ => is_current, // default to "current"
                    };

                    if should_show && !page.sections.is_empty() {
                        html.push_str("      <ul class=\"toc-sections\">\n");
                        for section in &page.sections {
                            html.push_str(&format!(
                                "        <li><a href=\"{}/{}#{}\">{}</a></li>\n",
                                self.base_path,
                                html_escape(&page.output_filename),
                                html_escape(&section.id),
                                html_escape(&section.title)
                            ));
                        }
                        html.push_str("      </ul>\n");
                    }

                    html.push_str("    </li>\n");
                }
            }
        }

        html.push_str("  </ul>\n");
        html.push_str("</nav>\n");
        html.push_str("<div id=\"content-wrapper\">\n");
        html
    }

    pub fn generate_wrapper_end() -> String {
        "</div> <!-- content-wrapper -->\n".to_string()
    }

    pub fn generate_css() -> String {
        r#"<style>
body {
  margin: 0;
  padding: 0;
  display: flex;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: background-color 0.3s, color 0.3s;
}

#toc-sidebar {
  width: 250px;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  overflow-y: auto;
  background: var(--bg-secondary);
  padding: 20px;
  border-right: 1px solid var(--border-color);
  box-sizing: border-box;
  transition: transform 0.3s ease;
}

#content-wrapper {
  margin-left: 250px;
  padding: 20px 40px;
  max-width: 900px;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-primary);
  transition: margin-left 0.3s ease;
}

.toc-header h2 {
  margin-top: 0;
  font-size: 1.5em;
  color: var(--text-primary);
}

.search-button {
  width: 100%;
  padding: 10px 12px;
  margin-bottom: 20px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 5px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.search-button:hover {
  background: var(--bg-hover);
  border-color: var(--text-secondary);
  color: var(--text-primary);
}

.search-button svg {
  flex-shrink: 0;
}

.toc-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.toc-list li {
  margin: 5px 0;
}

.toc-list a {
  text-decoration: none;
  margin: 0;
  padding: 0;
}

.toc-list li:not(.current) > a:hover {
  padding-left: 1.5rem;
}

.toc-list li.current > a {
  font-weight: bold;
}

.toc-list li.current > a:before {
    content: "▶ ";
}

/* Part (separator/heading) styling */
.toc-part {
  margin: 20px 0 8px 0;
  padding: 8px 12px;
  font-weight: bold;
  font-size: 0.9em;
  color: var(--bg-primary);
  background: var(--text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
}

.toc-part > a {
  color: inherit;
  background: inherit;
  text-decoration: none;
}

/* Chapter styling */
.toc-chapter {
  margin: 16px 0 6px 0;
  padding: 8px 12px;
  font-weight: bold;
  font-size: 0.9em;
  color: var(--bg-primary);
  background: var(--text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
  opacity: 0.8;
}

.toc-chapter > a {
  color: inherit;
  background: inherit;
  text-decoration: none;
}

/* Section styling */
.toc-section {
  margin: 12px 0 4px 0;
  padding: 8px 12px;
  font-weight: bold;
  font-size: 0.9em;
  color: var(--bg-primary);
  background: var(--text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
  opacity: 0.6;
}

.toc-section > a {
  color: inherit;
  background: inherit;
  text-decoration: none;
}

/* Subsection styling */
.toc-subsection {
  margin: 8px 0 4px 0;
  padding: 8px 12px;
  font-weight: bold;
  font-size: 0.9em;
  color: var(--bg-primary);
  background: var(--text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
  opacity: 0.4;
}

.toc-subsection > a {
  color: inherit;
  background: inherit;
  text-decoration: none;
}

/* Page (default) styling - 通常表示 */
.toc-page {
  margin: 4px 0;
}

.toc-page > a {
  display: block;
  padding: 8px 12px;
  text-decoration: none;
  color: var(--text-primary);
  border-radius: 4px;
  font-weight: 500;
  transition: background 0.2s;
}

/* Section (H2) styling */
.toc-sections {
  list-style: none;
  padding: 0;
  margin: 4px 0 0 0;
}

.toc-sections li {
  margin: 2px 0;
}

.toc-sections a {
  display: block;
  padding: 6px 12px 6px 24px;
  text-decoration: none;
  color: var(--text-secondary);
  border-radius: 4px;
  font-size: 0.9em;
  transition: background 0.2s;
}

.toc-sections a:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* Table styling */
table {
  border-collapse: collapse;
  width: 100%;
  margin: 1em 0;
  border: 1px solid var(--border-color);
}

table th,
table td {
  padding: 8px 12px;
  text-align: left;
  border: 1px solid var(--border-color);
}

table thead th {
  background: var(--bg-active);
  color: var(--text-primary);
  font-weight: bold;
}

table tbody tr:nth-child(even) {
  background: var(--bg-secondary);
}

table tbody tr:nth-child(odd) {
  background: var(--bg-primary);
}

table tbody tr:hover {
  background: var(--bg-hover);
}

@media (max-width: 768px) {
  #toc-sidebar {
    position: static;
    width: 100%;
    height: auto;
    border-right: none;
    border-bottom: 1px solid #ddd;
  }

  #content-wrapper {
    margin-left: 0;
  }
}
</style>"#
            .to_string()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_items() -> Vec<BookItem> {
        vec![
            BookItem {
                title: "Introduction".to_string(),
                level: ItemLevel::Page,
                page: Some(PageInfo {
                    source_path: std::path::PathBuf::from("src/intro.md"),
                    output_filename: "intro.html".to_string(),
                    sections: vec![],
                }),
            },
            BookItem {
                title: "Chapter 1".to_string(),
                level: ItemLevel::Page,
                page: Some(PageInfo {
                    source_path: std::path::PathBuf::from("src/chapter1.md"),
                    output_filename: "chapter1.html".to_string(),
                    sections: vec![],
                }),
            },
            BookItem {
                title: "Chapter 2".to_string(),
                level: ItemLevel::Page,
                page: Some(PageInfo {
                    source_path: std::path::PathBuf::from("src/chapter2.md"),
                    output_filename: "chapter2.html".to_string(),
                    sections: vec![],
                }),
            },
        ]
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("Hello"), "Hello");
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("A & B"), "A &amp; B");
        assert_eq!(html_escape("\"quote\""), "&quot;quote&quot;");
        assert_eq!(html_escape("'single'"), "&#39;single&#39;");
    }

    #[test]
    fn test_toc_generator_new() {
        let generator =
            TocGenerator::new("My Book".to_string(), "current".to_string(), "".to_string());
        assert_eq!(generator.book_title, "My Book");
    }

    #[test]
    fn test_generate_toc_html_no_current() {
        let generator = TocGenerator::new(
            "Test Book".to_string(),
            "current".to_string(),
            "".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);

        assert!(html.contains("<nav id=\"toc-sidebar\">"));
        assert!(html.contains("<h2>Test Book</h2>"));
        assert!(html.contains("Introduction"));
        assert!(html.contains("Chapter 1"));
        assert!(html.contains("Chapter 2"));
        assert!(html.contains("href=\"/intro.html\""));
        assert!(html.contains("href=\"/chapter1.html\""));
        assert!(html.contains("href=\"/chapter2.html\""));
        assert!(!html.contains("class=\"current\""));
        assert!(html.contains("<div id=\"content-wrapper\">"));
    }

    #[test]
    fn test_generate_toc_html_with_current() {
        let generator = TocGenerator::new(
            "Test Book".to_string(),
            "current".to_string(),
            "".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, Some("chapter1.html"));

        assert!(html.contains("href=\"/chapter1.html\""));
        assert!(html.contains("class=\"toc-page current\""));
        assert!(!html.contains("href=\"/intro.html\" class=\"current\""));
        assert!(!html.contains("href=\"/chapter2.html\" class=\"current\""));
    }

    #[test]
    fn test_generate_toc_html_escapes() {
        let generator = TocGenerator::new(
            "Test <Book>".to_string(),
            "current".to_string(),
            "".to_string(),
        );
        let items = vec![BookItem {
            title: "Chapter <1>".to_string(),
            level: ItemLevel::Page,
            page: Some(PageInfo {
                source_path: std::path::PathBuf::from("src/ch1.md"),
                output_filename: "ch1.html".to_string(),
                sections: vec![],
            }),
        }];
        let html = generator.generate_toc_html(&items, None);

        assert!(html.contains("Test &lt;Book&gt;"));
        assert!(html.contains("Chapter &lt;1&gt;"));
    }

    #[test]
    fn test_generate_wrapper_end() {
        let html = TocGenerator::generate_wrapper_end();
        assert!(html.contains("</div>"));
        assert!(html.contains("content-wrapper"));
    }

    #[test]
    fn test_generate_css() {
        let css = TocGenerator::generate_css();
        assert!(css.contains("<style>"));
        assert!(css.contains("#toc-sidebar"));
        assert!(css.contains("#content-wrapper"));
        assert!(css.contains("position: fixed"));
        assert!(css.contains("@media (max-width: 768px)"));
        assert!(css.contains("</style>"));
    }

    #[test]
    fn test_toc_links_correct() {
        let generator =
            TocGenerator::new("Test".to_string(), "current".to_string(), "".to_string());
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);

        // Check that all links are present and correctly formatted
        assert!(html.contains("<a href=\"/intro.html\">Introduction</a>"));
        assert!(html.contains("<a href=\"/chapter1.html\">Chapter 1</a>"));
        assert!(html.contains("<a href=\"/chapter2.html\">Chapter 2</a>"));
    }

    #[test]
    fn test_base_path_normalization() {
        // Test with base_path without leading or trailing slash
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "gnuplot-book".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with leading slash only
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "/gnuplot-book".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with trailing slash only
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "gnuplot-book/".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with both leading and trailing slash
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "/gnuplot-book/".to_string(),
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with empty base_path
        let generator =
            TocGenerator::new("Test".to_string(), "current".to_string(), "".to_string());
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/intro.html\">Introduction</a>"));
    }
}
