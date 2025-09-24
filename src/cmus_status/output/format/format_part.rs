use std::convert::TryFrom;

use super::FormatExpression;
use crate::cmus_status::data::CmusPlaybackStatus;
use crate::error::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub enum FormatPart {
    /// Just print the given text.
    /// This whole variant can be represented as a string.
    /// __Config example:__
    /// ```toml
    ///     format = "Hello from the status-line!"
    /// ```
    Text(String),

    /// Prints the currently playing song's name.
    Title,

    /// Prints the `CmusPlaybackStatus` of the playing song.
    Status,

    /// Prints the tag value for the given tag name.
    /// If the tag doesn't exist, prints nothing.
    Tag(String),

    /// TODO: Deprecated, use with `FormatPart::If` conditional.
    /// If the first argument's status is the current `CmusPlaybackStatus`,
    /// then, print the given string.
    /// The `CmusPlaybackStatus` can be one of:
    ///   - Playing
    ///   - Paused
    ///   - Stopped
    MatchStatus(CmusPlaybackStatus, String),

    /// Truncate the given `FormatPart` to the given length (`usize`).
    /// Max length is inclusive.
    /// __Config example:__
    /// ```toml
    ///     format = "%{ Truncate(Status, 60) }"
    /// ```
    Truncate(Box<FormatPart>, usize), // Inclusive

    /// Run `htmlescape::encode_minimal` on the wrapped
    /// `FormatPart`'s resulting string.
    HtmlEscape(Box<FormatPart>),

    /// Prints a ProgressBar with the given `ProgressBarConfig`.
    /// `ProgressBarConfig` can be a string such as:
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ ProgressBar("<###--->") }
    ///     """
    /// ```
    /// ... where the first and last characters (`<`,`>`) are used as the start and end
    /// characters of the bar, respectively. The second character in the string (`#`) is used
    /// as the "full" character, and the second to last as the "empty" (`-`) character.
    /// The "full" characters are printed if the playback percentage of the track has reached that
    /// point, the "empty" characters if it hasn't.
    /// The total length of the string is also the printed length.
    ProgressBar(ProgressBarConfig),

    /// A list of `FormatPart`s.
    /// Useful with `FormatPart::If`.
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ Container([
    ///         Text("Hello "),
    ///         Text("World! "),
    ///         Status,
    ///     ]) }
    ///     """
    /// ```
    Container(Vec<Box<FormatPart>>),

    /// `if` conditional. If the `FormatExpression` returns `true`,
    /// then `FormatPart` is printed.
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ If(IsStatus(Playing),
    ///        Text("Cmus is playing a song!")) }
    ///     """
    /// ```
    If(FormatExpression, Box<FormatPart>),

    /// `if/else` conditional. If the `FormatExpression` returns `true`,
    /// then the _first_ `FormatPart` is printed,
    /// otherwise the _second_ `FormatPart` is printed.
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ IfElse(
    ///         HasTag("artist"),
    ///         Tag("artist"),
    ///         Text("unknown artist")) }
    ///     """
    /// ```
    IfElse(FormatExpression, Box<FormatPart>, Box<FormatPart>),
}

impl From<Box<FormatPart>> for FormatPart {
    fn from(b: Box<FormatPart>) -> Self {
        *b
    }
}

impl<'a> From<Box<&'a FormatPart>> for &'a FormatPart {
    fn from(b: Box<&'a FormatPart>) -> Self {
        *b
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct ProgressBarConfig {
    pub start: Option<char>,
    pub end: Option<char>,
    pub full: char,
    pub empty: char,
    total_width: usize,
}

impl ProgressBarConfig {
    pub fn inner_width(&self) -> usize {
        self.total_width
            - if self.start.is_some() { 1 } else { 0 }
            - if self.end.is_some() { 1 } else { 0 }
    }

    pub fn text_with_filled(&self, filled_characters: usize) -> String {
        assert!(self.total_width > filled_characters);

        let mut s = String::new();
        if let Some(start) = self.start {
            s.push(start);
        }
        s.push_str(self.full.to_string().repeat(filled_characters).as_str());
        s.push_str(
            self.empty
                .to_string()
                .repeat(self.inner_width() - filled_characters)
                .as_str(),
        );
        if let Some(end) = self.end {
            s.push(end);
        }
        s
    }
}

impl TryFrom<String> for ProgressBarConfig {
    type Error = Error;
    fn try_from(s: String) -> MyResult<Self> {
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();
        if len < 2 {
            Err(Error::ProgressBarConfigMinLen(2, s))
        } else if len == 2 {
            Ok(ProgressBarConfig {
                start: None,
                end: None,
                full: *chars.get(0).unwrap(),
                empty: *chars.get(1).unwrap(),
                total_width: len,
            })
        } else if len == 3 {
            Ok(ProgressBarConfig {
                start: Some(*chars.get(0).unwrap()),
                end: None,
                full: *chars.get(1).unwrap(),
                empty: *chars.get(2).unwrap(),
                total_width: len,
            })
        } else {
            Ok(ProgressBarConfig {
                start: Some(*chars.get(0).unwrap()),
                end: Some(*chars.get(len - 1).unwrap()),
                full: *chars.get(1).unwrap(),
                empty: *chars.get(len - 2).unwrap(),
                total_width: len,
            })
        }
    }
}
