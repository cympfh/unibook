use crate::book::BookItem;
#[cfg(test)]
use crate::book::PageInfo;

pub struct TocGenerator {
    book_title: String,
    show_sections: String,
    base_path: String,
    foldlevel: u8,
}

impl TocGenerator {
    pub fn new(
        book_title: String,
        show_sections: String,
        base_path: String,
        foldlevel: u8,
    ) -> Self {
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
            foldlevel,
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
            match item {
                BookItem::Part {
                    title,
                    level,
                    children,
                } => {
                    // Check if current page is in this part's children
                    let contains_current = current_page.is_some_and(|current| {
                        children.iter().any(|p| p.output_filename == current)
                    });

                    // Check if this part should be folded by default
                    // Don't fold if it contains the current page
                    let should_fold =
                        self.foldlevel > 0 && *level >= self.foldlevel && !contains_current;
                    let fold_class = if should_fold && !children.is_empty() {
                        " foldable"
                    } else {
                        ""
                    };
                    let collapsed_class = if should_fold && !children.is_empty() {
                        " collapsed"
                    } else {
                        ""
                    };

                    // Render part as a separator/heading (no link)
                    // Use level-specific CSS class
                    let class = format!("toc-part toc-part-level-{}{}", level, fold_class);

                    // Add toggle button if has children
                    if !children.is_empty() {
                        html.push_str(&format!(
                            "    <li class=\"{}\">\n      <div class=\"toc-part-header{}\">\n        <button class=\"toc-fold-toggle\" aria-label=\"Toggle section\">\n          <svg class=\"fold-icon\" width=\"12\" height=\"12\" viewBox=\"0 0 12 12\">\n            <path d=\"M3 4.5l3 3 3-3\" stroke=\"currentColor\" stroke-width=\"1.5\" fill=\"none\" stroke-linecap=\"round\"/>\n          </svg>\n        </button>\n        <span>{}</span>\n      </div>\n",
                            class,
                            collapsed_class,
                            html_escape(title)
                        ));
                    } else {
                        html.push_str(&format!(
                            "    <li class=\"{}\">{}</li>\n",
                            class,
                            html_escape(title)
                        ));
                    }

                    // Render child pages with indentation in a collapsible container
                    if !children.is_empty() {
                        html.push_str(&format!(
                            "      <ul class=\"toc-children{}\">\n",
                            collapsed_class
                        ));
                        for page in children {
                            let is_current = Some(page.output_filename.as_str()) == current_page;
                            let current_class = if is_current { " current" } else { "" };
                            let indent_class = format!("toc-page-child toc-page-indent-{}", level);

                            html.push_str(&format!(
                                "        <li>\n          <a href=\"{}/{}\" class=\"{}{}\">{}</a>\n",
                                self.base_path,
                                html_escape(&page.output_filename),
                                indent_class,
                                current_class,
                                html_escape(&page.title)
                            ));

                            // Add sections based on show_sections setting
                            let should_show = match self.show_sections.as_str() {
                                "always" => true,
                                "current" => is_current,
                                "never" => false,
                                _ => is_current, // default to "current"
                            };

                            if should_show && !page.sections.is_empty() {
                                // Apply indent class to sections based on parent part level
                                let sections_class =
                                    format!("toc-sections toc-sections-indent-{}", level);
                                html.push_str(&format!(
                                    "          <ul class=\"{}\">\n",
                                    sections_class
                                ));
                                for section in &page.sections {
                                    html.push_str(&format!(
                                        "            <li><a href=\"{}/{}#{}\">{}</a></li>\n",
                                        self.base_path,
                                        html_escape(&page.output_filename),
                                        html_escape(&section.id),
                                        html_escape(&section.title)
                                    ));
                                }
                                html.push_str("          </ul>\n");
                            }

                            html.push_str("        </li>\n");
                        }

                        // Close children container
                        html.push_str("      </ul>\n");
                        html.push_str("    </li>\n");
                    }
                }
                BookItem::Page(page) => {
                    // Top-level page (not under any part)
                    let is_current = Some(page.output_filename.as_str()) == current_page;
                    let current_class = if is_current { " class=\"current\"" } else { "" };
                    html.push_str(&format!(
                        "    <li>\n      <a href=\"{}/{}\"{}>{}</a>\n",
                        self.base_path,
                        html_escape(&page.output_filename),
                        current_class,
                        html_escape(&page.title)
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

        // Add JavaScript for fold/unfold functionality
        html.push_str("<script>\n");
        html.push_str("(function() {\n");
        html.push_str(
            "  document.querySelectorAll('.toc-part-header').forEach(function(header) {\n",
        );
        html.push_str("    header.addEventListener('click', function(e) {\n");
        html.push_str("      e.preventDefault();\n");
        html.push_str("      e.stopPropagation();\n");
        html.push_str("      var li = header.parentElement;\n");
        html.push_str("      var children = li.querySelector('.toc-children');\n");
        html.push_str("      if (children) {\n");
        html.push_str("        var isCollapsed = header.classList.contains('collapsed');\n");
        html.push_str("        if (isCollapsed) {\n");
        html.push_str("          header.classList.remove('collapsed');\n");
        html.push_str("          children.classList.remove('collapsed');\n");
        html.push_str("        } else {\n");
        html.push_str("          header.classList.add('collapsed');\n");
        html.push_str("          children.classList.add('collapsed');\n");
        html.push_str("        }\n");
        html.push_str("      }\n");
        html.push_str("    });\n");
        html.push_str("  });\n");
        html.push_str("})();\n");
        html.push_str("</script>\n");

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

.toc-list > li > a {
  display: block;
  padding: 8px 12px;
  text-decoration: none;
  color: var(--text-primary);
  border-radius: 4px;
  font-weight: 500;
  transition: background 0.2s;
}

.toc-list a:hover {
  background: var(--bg-hover);
}

.toc-list a.current {
  background: var(--bg-active);
  font-weight: bold;
  color: var(--text-primary);
}

/* Part (separator/heading) styling */
.toc-part {
  margin: 20px 0 8px 0;
  padding: 0;
  font-weight: bold;
  font-size: 0.9em;
  list-style: none;
}

.toc-part-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  color: var(--text-primary);
  background: transparent;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
  cursor: pointer;
  user-select: none;
  transition: background 0.2s ease;
}

.toc-part-header:hover {
  background: var(--bg-hover);
}

/* Level 1 specific: inverted colors */
.toc-part-level-1 .toc-part-header {
  color: var(--bg-primary);
  background: var(--text-primary);
}

.toc-part-level-1 .toc-part-header:hover {
  background: var(--text-secondary);
}

.toc-part-header span {
  flex: 1;
}

.toc-fold-toggle {
  background: none;
  border: none;
  padding: 0;
  margin: 0;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  color: currentColor;
  transition: transform 0.2s ease;
}

.toc-fold-toggle:hover {
  transform: scale(1.1);
}

.toc-fold-toggle .fold-icon {
  transform: rotate(0deg);
  transition: transform 0.2s ease;
}

.toc-part-header.collapsed .fold-icon {
  transform: rotate(-90deg);
}

.toc-children {
  list-style: none;
  padding: 0;
  margin: 4px 0 0 0;
  max-height: 10000px;
  overflow: hidden;
  transition: max-height 0.3s ease, opacity 0.3s ease;
  opacity: 1;
}

.toc-children.collapsed {
  max-height: 0;
  opacity: 0;
}

/* Part without children (no header wrapper) */
.toc-part:not(:has(.toc-part-header)) {
  padding: 8px 12px;
  color: var(--text-primary);
  background: transparent;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-radius: 4px;
}

/* Level 1: 大見出し (Part) - inverted colors for parts without children */
.toc-part-level-1:not(:has(.toc-part-header)) {
  color: var(--bg-primary);
  background: var(--text-primary);
}

/* Level 2: 中見出し (Chapter) */
.toc-part-level-2 {
  margin-left: 12px !important;
}

/* Level 3: 小見出し (Section) */
.toc-part-level-3 {
  margin-left: 20px !important;
}

/* Child page indentation */
.toc-page-child {
  display: block;
  padding: 8px 12px;
  text-decoration: none;
  color: var(--text-primary);
  border-radius: 4px;
  font-weight: 500;
  transition: background 0.2s;
}

.toc-page-indent-1 {
  padding-left: 16px !important;
}

.toc-page-indent-2 {
  padding-left: 24px !important;
}

.toc-page-indent-3 {
  padding-left: 32px !important;
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

/* Additional indentation for sections under child pages */
.toc-sections-indent-1 a {
  padding-left: 40px;
}

.toc-sections-indent-2 a {
  padding-left: 48px;
}

.toc-sections-indent-3 a {
  padding-left: 56px;
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

fn convert_math_delimiters(s: &str) -> String {
    let mut result = String::new();
    let mut in_dollar = false;
    let mut buffer = String::new();

    for ch in s.chars() {
        if ch == '$' {
            if in_dollar {
                // Closing $, convert buffer content
                result.push_str("\\(");
                result.push_str(&buffer);
                result.push_str("\\)");
                buffer.clear();
                in_dollar = false;
            } else {
                // Opening $
                in_dollar = true;
            }
        } else if in_dollar {
            buffer.push(ch);
        } else {
            result.push(ch);
        }
    }

    // If still in_dollar at the end, it means unclosed $
    // Just append the remaining content as-is
    if in_dollar {
        result.push('$');
        result.push_str(&buffer);
    }

    result
}

fn html_escape(s: &str) -> String {
    // First convert math delimiters, then escape HTML
    let converted = convert_math_delimiters(s);
    converted
        .replace('&', "&amp;")
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
            BookItem::Page(PageInfo {
                title: "Introduction".to_string(),
                source_path: std::path::PathBuf::from("src/intro.md"),
                output_filename: "intro.html".to_string(),
                sections: vec![],
            }),
            BookItem::Page(PageInfo {
                title: "Chapter 1".to_string(),
                source_path: std::path::PathBuf::from("src/chapter1.md"),
                output_filename: "chapter1.html".to_string(),
                sections: vec![],
            }),
            BookItem::Page(PageInfo {
                title: "Chapter 2".to_string(),
                source_path: std::path::PathBuf::from("src/chapter2.md"),
                output_filename: "chapter2.html".to_string(),
                sections: vec![],
            }),
        ]
    }

    #[test]
    fn test_convert_math_delimiters() {
        assert_eq!(convert_math_delimiters("Hello"), "Hello");
        assert_eq!(convert_math_delimiters("$x^2$"), "\\(x^2\\)");
        assert_eq!(
            convert_math_delimiters("Text $a+b$ more"),
            "Text \\(a+b\\) more"
        );
        assert_eq!(
            convert_math_delimiters("$x$ and $y$"),
            "\\(x\\) and \\(y\\)"
        );
        assert_eq!(convert_math_delimiters("$~$"), "\\(~\\)");
        // Test unclosed dollar
        assert_eq!(convert_math_delimiters("$unclosed"), "$unclosed");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("Hello"), "Hello");
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("A & B"), "A &amp; B");
        assert_eq!(html_escape("\"quote\""), "&quot;quote&quot;");
        assert_eq!(html_escape("'single'"), "&#39;single&#39;");
        // Test math conversion with HTML escaping
        assert_eq!(html_escape("$x^2$"), "\\(x^2\\)");
        assert_eq!(html_escape("Title $~$ Test"), "Title \\(~\\) Test");
    }

    #[test]
    fn test_toc_generator_new() {
        let generator = TocGenerator::new(
            "My Book".to_string(),
            "current".to_string(),
            "".to_string(),
            0,
        );
        assert_eq!(generator.book_title, "My Book");
    }

    #[test]
    fn test_generate_toc_html_no_current() {
        let generator = TocGenerator::new(
            "Test Book".to_string(),
            "current".to_string(),
            "".to_string(),
            0,
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
            0,
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, Some("chapter1.html"));

        assert!(html.contains("href=\"/chapter1.html\" class=\"current\""));
        assert!(!html.contains("href=\"/intro.html\" class=\"current\""));
        assert!(!html.contains("href=\"/chapter2.html\" class=\"current\""));
    }

    #[test]
    fn test_generate_toc_html_escapes() {
        let generator = TocGenerator::new(
            "Test <Book>".to_string(),
            "current".to_string(),
            "".to_string(),
            0,
        );
        let items = vec![BookItem::Page(PageInfo {
            title: "Chapter <1>".to_string(),
            source_path: std::path::PathBuf::from("src/ch1.md"),
            output_filename: "ch1.html".to_string(),
            sections: vec![],
        })];
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
            TocGenerator::new("Test".to_string(), "current".to_string(), "".to_string(), 0);
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
            0,
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with leading slash only
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "/gnuplot-book".to_string(),
            0,
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with trailing slash only
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "gnuplot-book/".to_string(),
            0,
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with base_path with both leading and trailing slash
        let generator = TocGenerator::new(
            "Test".to_string(),
            "current".to_string(),
            "/gnuplot-book/".to_string(),
            0,
        );
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/gnuplot-book/intro.html\">Introduction</a>"));

        // Test with empty base_path
        let generator =
            TocGenerator::new("Test".to_string(), "current".to_string(), "".to_string(), 0);
        let items = create_test_items();
        let html = generator.generate_toc_html(&items, None);
        assert!(html.contains("<a href=\"/intro.html\">Introduction</a>"));
    }
}
