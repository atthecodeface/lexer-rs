//a Imports
use crate::{PosnInCharStream, TokenTypeError};

//a TokenParseError
//tp TokenParseError
/// A simple implementation of a type supporting TokenTypeError
///
/// An error in parsing a token - most often an 'unrecognized character'
///
/// P : PosnInCharStream
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenParseError<P>
where
    P: PosnInCharStream,
{
    ch: char,
    pos: P,
}

//ip Error for TokenParseError
impl<P> std::error::Error for TokenParseError<P> where P: PosnInCharStream {}

impl<P> TokenTypeError<P> for TokenParseError<P>
where
    P: PosnInCharStream,
{
    fn failed_to_parse(ch: char, pos: P) -> Self {
        Self { ch, pos }
    }
}

//ip Display for TokenParseError
impl<P> std::fmt::Display for TokenParseError<P>
where
    P: PosnInCharStream,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Failed to parse: unexpected char '{}' at ", self.ch)?;
        self.pos.error_fmt(fmt)
    }
}

