use crate::cmus_status::data::CmusPlaybackStatus;

#[derive(Deserialize)]
pub enum FormatExpression {
    /// Always returns `true`.
    True,
    /// Always returns `false`.
    False,
    /// Returns `true` if both of the given expressions are `true`.
    And(Box<FormatExpression>, Box<FormatExpression>),
    /// Returns `true` if either of the given expressions are `true`.
    Or(Box<FormatExpression>, Box<FormatExpression>),
    /// Inverts the given expression.
    Not(Box<FormatExpression>),
    /// Returns `true` if the given `CmusPlaybackStatus`
    /// is the currently playing song's `CmusPlaybackStatus`.
    IsStatus(CmusPlaybackStatus),
    /// Returns `true` if the given tag is set for the current track.
    HasTag(String),
}
