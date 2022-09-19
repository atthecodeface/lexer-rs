//a Imports
use crate::{LexerError, PosnInCharStream};

//a LexerParseError
//tp LexerParseError
/// A simple implementation of a type supporting TokenTypeError
///
/// An error in parsing a token - most often an 'unrecognized character'
///
/// P : PosnInCharStream
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerParseError<P>
where
    P : PosnInCharStream,
{
    ch: char,
    pos: P,
}

//ip Error for LexerParseError
impl<P> std::error::Error for LexerParseError<P>
where
    P : PosnInCharStream,
{
}

//ip LexerError for LexerParseError
impl<P> LexerError<P> for  LexerParseError<P>
where
    P : PosnInCharStream,
{
    fn failed_to_parse(pos: P, ch: char) -> Self {
        Self { ch, pos }
    }
}

//ip Display for LexerParseError
impl<P> std::fmt::Display for  LexerParseError<P>
where
    P : PosnInCharStream,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Failed to parse: unexpected char '{}' at ", self.ch)?;
        self.pos.error_fmt(fmt)
    }
}
