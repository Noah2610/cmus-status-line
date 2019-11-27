use super::*;

#[derive(Deserialize)]
pub enum FormatPart {
    /// Just print the given text, which is never encoded with htmlescape.
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
    ///     format = "%{ MaxLen(Status, 60) }"
    /// ```
    MaxLen(Box<FormatPart>, usize), // Inclusive

    /// Prints a ProgressBar with the given `ProgressBarConfig`.
    /// `ProgressBarConfig` can be a string such as:
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ ProgressBar("<###--->") }
    ///     """
    /// ```
    /// ... where the first and last characters (`<,``>`) are used as the start and end
    /// characters of the bar, respectively. The second character in the string (`#`) is used
    /// as the "full" character, and the second to last as the "empty" (`-`) character.
    /// The "full" characters are printed if the playback percentage of the track has reached that
    /// point, the "empty" characters if it hasn't.
    /// The total length of the string is also the printed length.
    ProgressBar(ProgressBarConfig),

    /// TODO: Documentation
    /// A list of `FormatPart`s.
    /// Useful with `FormatPart::If`.
    // Block(Vec<Box<FormatPart>>),

    /// `If` conditional. If the `FormatExpression` returns `true`,
    /// then `FormatPart` is printed.
    /// __Config example:__
    /// ```toml
    ///     format = """
    ///     %{ If(IsStatus(Playing),
    ///        "Cmus is playing a song!") }
    ///     """
    /// ```
    If(FormatExpression, Box<FormatPart>),
}

#[derive(Deserialize)]
pub enum FormatExpression {
    /// Returns `true` if the given `CmusPlaybackStatus`
    /// is the currently playing's song `CmusPlaybackStatus`.
    IsStatus(CmusPlaybackStatus),
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct ProgressBarConfig {
    pub start:   Option<char>,
    pub end:     Option<char>,
    pub full:    char,
    pub empty:   char,
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
        let len = s.len();
        if len < 2 {
            Err(Error::ProgressBarConfigMinLen(2, s))
        } else if len == 2 {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       None,
                end:         None,
                full:        *chars.get(0).unwrap(),
                empty:       *chars.get(1).unwrap(),
                total_width: len,
            })
        } else if len == 3 {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       Some(*chars.get(0).unwrap()),
                end:         None,
                full:        *chars.get(1).unwrap(),
                empty:       *chars.get(2).unwrap(),
                total_width: len,
            })
        } else {
            let chars = s.chars().collect::<Vec<char>>();
            Ok(ProgressBarConfig {
                start:       Some(*chars.get(0).unwrap()),
                end:         Some(*chars.get(len - 1).unwrap()),
                full:        *chars.get(1).unwrap(),
                empty:       *chars.get(len - 2).unwrap(),
                total_width: len,
            })
        }
    }
}
