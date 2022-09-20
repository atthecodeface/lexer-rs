//a Imports
use crate::{LexerError, PosnInStream};

//a SimpleParseError
//tp SimpleParseError
/// A simple implementation of a type supporting LexerError
///
/// An error in parsing a token
///
/// P : PosnInStream
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleParseError<P>
where
    P: PosnInStream,
{
    pub ch: char,
    pub pos: P,
}

//ip Error for SimpleParseError
impl<P> std::error::Error for SimpleParseError<P> where P: PosnInStream {}

//ip LexerError for SimpleParseError
impl<P> LexerError<P> for SimpleParseError<P>
where
    P: PosnInStream,
{
    fn failed_to_parse(pos: P, ch: char) -> Self {
        Self { ch, pos }
    }
}

//ip Display for SimpleParseError
impl<P> std::fmt::Display for SimpleParseError<P>
where
    P: PosnInStream,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Failed to parse: unexpected char '{}' at ", self.ch)?;
        self.pos.error_fmt(fmt)
    }
}
