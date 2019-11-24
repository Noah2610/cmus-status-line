pub struct Settings {
    pub escape_html: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { escape_html: false }
    }
}
