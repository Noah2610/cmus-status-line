use super::*;

#[derive(Deserialize)]
pub enum FormatPart {
    Text(String),
    Title,
    Status,
    MatchStatus(CmusPlaybackStatus, String),
    MaxLen(usize, Box<FormatPart>), // Inclusive
    ProgressBar(ProgressBarConfig),
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
