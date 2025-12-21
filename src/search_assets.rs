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
}
