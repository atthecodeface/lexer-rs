//a PosnIn traits
//tt PosnInStream
/// Trait for location within a stream
///
/// This base trait is used to enable tracking the position of a token
/// parser within a stream in a manner that is useful for
/// human-readable error messages
///
/// A simple implementation can be NULL, if the position is not
/// critical for error messages for the token parser - for example,
/// parsing a simple string in a test.
///
/// For a single file implementation see [crate::LineColumn]
pub trait PosnInStream:
    Sized + std::fmt::Debug + Copy + std::default::Default + PartialEq + Eq + std::hash::Hash
{
    fn advance_cols(self, _byte_ofs: usize, _num_chars: usize) -> Self {
        self
    }
    fn advance_line(self, _byte_ofs: usize) -> Self {
        self
    }
    fn error_fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, fmt)
    }
}

//ip PosnInStream for ()
impl PosnInStream for () {}

//tt PosnInCharStream
/// Trait for location within a character stream
///
/// This tracks a byte offset within the stream so that strings can be
/// retrieved from the stream. Byte offsets *must* always be on UTF8
/// boundaries.
pub trait PosnInCharStream: PosnInStream {
    fn byte_ofs(&self) -> usize;
}

//ip PosnInCharStream for usize
impl PosnInStream for usize {
    fn advance_cols(self, byte_ofs: usize, _num_chars: usize) -> Self {
        self + byte_ofs
    }
    fn advance_line(self, byte_ofs: usize) -> Self {
        self + byte_ofs
    }
}
impl PosnInCharStream for usize {
    fn byte_ofs(&self) -> usize {
        *self
    }
}

//a Tokens
//tt TokenType
/// The traits required of a token
pub trait TokenType: Sized + std::fmt::Debug + Copy {}

//ip TokenType for char and u*
impl TokenType for char {}
impl TokenType for u8 {}
impl TokenType for u16 {}
impl TokenType for u32 {}
impl TokenType for usize {}

//tt TokenTypeError
/// A trait required of an error - a char that does not match any
/// token parser rust return an error, and this trait requires that
/// such an error be provided
pub trait TokenTypeError<P: PosnInCharStream>: Sized + std::error::Error {
    fn failed_to_parse(ch: char, pos: P) -> Self;
}
