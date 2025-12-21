pub struct SearchAssets;

impl SearchAssets {
    pub fn javascript() -> &'static str {
        include_str!("../assets/search.js")
    }

    pub fn css() -> &'static str {
        include_str!("../assets/search.css")
    }

    pub fn html() -> &'static str {
        include_str!("../assets/search.html")
    }

    pub fn theme_css() -> &'static str {
        include_str!("../assets/themes.css")
    }

    pub fn theme_switcher_html() -> &'static str {
        include_str!("../assets/theme-switcher.html")
    }

    pub fn theme_switcher_css() -> &'static str {
        include_str!("../assets/theme-switcher.css")
    }

    pub fn theme_switcher_js() -> &'static str {
        include_str!("../assets/theme-switcher.js")
    }

    pub fn toc_toggle_html() -> &'static str {
        include_str!("../assets/toc-toggle.html")
    }

    pub fn toc_toggle_css() -> &'static str {
        include_str!("../assets/toc-toggle.css")
    }

    pub fn toc_toggle_js() -> &'static str {
        include_str!("../assets/toc-toggle.js")
    }

    pub fn page_controls_start() -> &'static str {
        include_str!("../assets/page-controls-start.html")
    }

    pub fn page_controls_end() -> &'static str {
        include_str!("../assets/page-controls-end.html")
    }
}
