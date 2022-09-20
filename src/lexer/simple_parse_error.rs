//a Imports
use crate::{LexerError, UserPosn};

//a SimpleParseError
//tp SimpleParseError
/// A simple implementation of a type supporting LexerError
///
/// An error in parsing a token
///
/// P : UserPosn
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleParseError<P>
where
    P: UserPosn,
{
    /// The character which could not be matched to a token
    pub ch: char,

    /// The position of the character in the stream
    pub pos: P,
}

//ip Error for SimpleParseError
impl<P> std::error::Error for SimpleParseError<P> where P: UserPosn {}

//ip LexerError for SimpleParseError
impl<P> LexerError<P> for SimpleParseError<P>
where
    P: UserPosn,
{
    fn failed_to_parse(pos: P, ch: char) -> Self {
        Self { ch, pos }
    }
}

//ip Display for SimpleParseError
impl<P> std::fmt::Display for SimpleParseError<P>
where
    P: UserPosn,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Failed to parse: unexpected char '{}' at ", self.ch)?;
        self.pos.error_fmt(fmt)
    }
}
