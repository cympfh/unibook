use crate::book::PageInfo;

pub struct TocGenerator {
    book_title: String,
}

impl TocGenerator {
    pub fn new(book_title: String) -> Self {
        Self { book_title }
    }

    pub fn generate_toc_html(&self, pages: &[PageInfo], current_page: Option<&str>) -> String {
        let mut html = String::new();
        html.push_str("<nav id=\"toc-sidebar\">\n");
        html.push_str(&format!(
            "  <div class=\"toc-header\"><h2>{}</h2></div>\n",
            html_escape(&self.book_title)
        ));
        html.push_str("  <ul class=\"toc-list\">\n");

        for page in pages {
            let current_class = if Some(page.output_filename.as_str()) == current_page {
                " class=\"current\""
            } else {
                ""
            };
            html.push_str(&format!(
                "    <li><a href=\"{}\"{}>{}</a></li>\n",
                html_escape(&page.output_filename),
                current_class,
                html_escape(&page.title)
            ));
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
}

#toc-sidebar {
  width: 250px;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  overflow-y: auto;
  background: #f4f4f4;
  padding: 20px;
  border-right: 1px solid #ddd;
  box-sizing: border-box;
}

#content-wrapper {
  margin-left: 250px;
  padding: 20px 40px;
  max-width: 900px;
  width: 100%;
  box-sizing: border-box;
}

.toc-header h2 {
  margin-top: 0;
  font-size: 1.5em;
  color: #333;
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
  display: block;
  padding: 8px 12px;
  text-decoration: none;
  color: #333;
  border-radius: 4px;
  transition: background 0.2s;
}

.toc-list a:hover {
  background: #e0e0e0;
}

.toc-list a.current {
  background: #d0d0d0;
  font-weight: bold;
  color: #000;
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

    fn create_test_pages() -> Vec<PageInfo> {
        vec![
            PageInfo {
                title: "Introduction".to_string(),
                source_path: std::path::PathBuf::from("src/intro.md"),
                output_filename: "intro.html".to_string(),
            },
            PageInfo {
                title: "Chapter 1".to_string(),
                source_path: std::path::PathBuf::from("src/chapter1.md"),
                output_filename: "chapter1.html".to_string(),
            },
            PageInfo {
                title: "Chapter 2".to_string(),
                source_path: std::path::PathBuf::from("src/chapter2.md"),
                output_filename: "chapter2.html".to_string(),
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
        let generator = TocGenerator::new("My Book".to_string());
        assert_eq!(generator.book_title, "My Book");
    }

    #[test]
    fn test_generate_toc_html_no_current() {
        let generator = TocGenerator::new("Test Book".to_string());
        let pages = create_test_pages();
        let html = generator.generate_toc_html(&pages, None);

        assert!(html.contains("<nav id=\"toc-sidebar\">"));
        assert!(html.contains("<h2>Test Book</h2>"));
        assert!(html.contains("Introduction"));
        assert!(html.contains("Chapter 1"));
        assert!(html.contains("Chapter 2"));
        assert!(html.contains("href=\"intro.html\""));
        assert!(html.contains("href=\"chapter1.html\""));
        assert!(html.contains("href=\"chapter2.html\""));
        assert!(!html.contains("class=\"current\""));
        assert!(html.contains("<div id=\"content-wrapper\">"));
    }

    #[test]
    fn test_generate_toc_html_with_current() {
        let generator = TocGenerator::new("Test Book".to_string());
        let pages = create_test_pages();
        let html = generator.generate_toc_html(&pages, Some("chapter1.html"));

        assert!(html.contains("href=\"chapter1.html\" class=\"current\""));
        assert!(!html.contains("href=\"intro.html\" class=\"current\""));
        assert!(!html.contains("href=\"chapter2.html\" class=\"current\""));
    }

    #[test]
    fn test_generate_toc_html_escapes() {
        let generator = TocGenerator::new("Test <Book>".to_string());
        let pages = vec![PageInfo {
            title: "Chapter <1>".to_string(),
            source_path: std::path::PathBuf::from("src/ch1.md"),
            output_filename: "ch1.html".to_string(),
        }];
        let html = generator.generate_toc_html(&pages, None);

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
        let generator = TocGenerator::new("Test".to_string());
        let pages = create_test_pages();
        let html = generator.generate_toc_html(&pages, None);

        // Check that all links are present and correctly formatted
        assert!(html.contains("<a href=\"intro.html\">Introduction</a>"));
        assert!(html.contains("<a href=\"chapter1.html\">Chapter 1</a>"));
        assert!(html.contains("<a href=\"chapter2.html\">Chapter 2</a>"));
    }
}
