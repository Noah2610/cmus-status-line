const DEFAULT_FORMAT: &str = "DEFAULT-%title";

pub struct Format {
    parts: Vec<FormatPart>,
}

pub enum FormatPart {
    Text(&'static str),
}

impl From<&str> for Format {
    fn from(string: &str) -> Self {
        Self { parts: Vec::new() }
    }
}

impl Default for Format {
    fn default() -> Self {
        DEFAULT_FORMAT.into()
    }
}
