mod builder;
mod format;

pub use format::prelude::*;

use super::data::prelude::*;
use crate::error::prelude::*;
use std::fmt;

use crate::config::OutputConfig;
use builder::StatusOutputBuilder;

const OVERFLOW_STR: &str = "...";

pub struct StatusOutput {
    data:   CmusData,
    format: Format,
    config: OutputConfig,
}

impl StatusOutput {
    pub fn builder() -> StatusOutputBuilder {
        StatusOutputBuilder::default()
    }

    fn get_format_text_for_parts<'a>(
        &self,
        parts: Vec<&'a FormatPart>,
    ) -> String {
        parts
            .iter()
            .filter_map(|part| self.get_format_text(part))
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_format_text(&self, part: &FormatPart) -> Option<String> {
        let mut maybe_escape_html = true;

        match part {
            FormatPart::Text(text) => {
                maybe_escape_html = false; // Never escape literal text
                Some(text.to_string())
            }

            FormatPart::Title => self.data.get_title(),

            FormatPart::Status => Some(self.data.get_status().to_string()),

            // TODO: Deprecated
            FormatPart::MatchStatus(playback_status, text) => {
                if self.data.is_status(playback_status) {
                    Some(text.to_string())
                } else {
                    None
                }
            }

            FormatPart::MaxLen(format_part_inner, max) => {
                maybe_escape_html = false; // Never escape FormatParts which hold another FormatPart
                let max = *max;
                self.get_format_text(format_part_inner.as_ref())
                    .map(|text| {
                        let mut text = text.to_string();
                        if text.len() > max {
                            let overflow_str_len = OVERFLOW_STR.len();
                            if max >= overflow_str_len * 2 {
                                text.truncate(max - overflow_str_len);
                                text.push_str(OVERFLOW_STR);
                            } else {
                                text.truncate(max);
                            }
                        }
                        text
                    })
            }

            FormatPart::ProgressBar(bar_config) => {
                if let Some(time) = self.data.get_time() {
                    let width = bar_config.inner_width();
                    let percent_complete = time.completion_percentage();
                    let characters =
                        (width as f32 * percent_complete).round() as usize;
                    Some(bar_config.text_with_filled(characters))
                } else {
                    None
                }
            }

            FormatPart::Container(format_parts_inner) => {
                maybe_escape_html = false; // Never escape FormatParts which hold another FormatPart
                Some(
                    self.get_format_text_for_parts(
                        format_parts_inner
                            .iter()
                            .map(std::ops::Deref::deref)
                            .collect(),
                    ),
                )
            }

            FormatPart::If(expression, format_part_inner) => {
                maybe_escape_html = false; // Never escape FormatParts which hold another FormatPart
                if self.is_expression_true(expression) {
                    self.get_format_text(format_part_inner)
                } else {
                    None
                }
            }
        }
        .map(|s| {
            if maybe_escape_html {
                self.maybe_escape_html(s.as_str())
            } else {
                s
            }
        })
    }

    fn is_expression_true(&self, expression: &FormatExpression) -> bool {
        match expression {
            FormatExpression::True => true,
            FormatExpression::False => false,

            FormatExpression::And(expr_one, expr_two) => {
                self.is_expression_true(expr_one)
                    && self.is_expression_true(expr_two)
            }

            FormatExpression::Or(expr_one, expr_two) => {
                self.is_expression_true(expr_one)
                    || self.is_expression_true(expr_two)
            }

            FormatExpression::Not(expr) => !self.is_expression_true(expr),

            FormatExpression::IsStatus(playback_status) => {
                self.data.is_status(&playback_status)
            }
        }
    }

    fn maybe_escape_html(&self, text: &str) -> String {
        if self.config.escape_html {
            htmlescape::encode_minimal(text)
        } else {
            text.into()
        }
    }
}

impl fmt::Display for StatusOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.get_format_text_for_parts(self.format.iter().collect())
        )
    }
}
