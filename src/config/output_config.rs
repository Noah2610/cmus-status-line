#[derive(Deserialize)]
pub struct OutputConfig {
    pub escape_html: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self { escape_html: false }
    }
}
