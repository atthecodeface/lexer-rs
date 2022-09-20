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
    //fp advance_cols
    /// Advance the state of the stream by a number of bytes and a
    /// number of characters; the characters are guaranteed to *not*
    /// be newlines
    ///
    /// For character streams (where num_bytes is not the same as
    /// num_char) this *must* only be invoked to move on to a new UTF8
    /// character boundary - hence num_bytes must be a (sum of)
    /// len_utf8 values for the text at byte offset of self.
    fn advance_cols(self, _num_bytes: usize, _num_chars: usize) -> Self {
        self
    }

    /// Advance the state of the stream by a number of bytes and to
    /// the start of the next line
    ///
    /// For character streams this *must* only be invoked to move on
    /// to a new UTF8 character boundary - hence num_bytes must be a
    /// (sum of) len_utf8 values for the text at byte offset of self,
    /// the last character of which is a newline
    fn advance_line(self, _num_bytes: usize) -> Self {
        self
    }

    /// Format self for an error - this can be the same format as
    /// Display (if implemented), or Debug, or whatever is desired
    ///
    /// It is required for a Lexer to generate a fail-to-parse-character
    /// error
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
    //mp byte_ofs
    /// Return the byte offset into the stream of the position.
    ///
    /// This must *always* be a UTF8 character boundary; it will be so
    fn byte_ofs(&self) -> usize;
}

//ip PosnInStream for usize
impl PosnInStream for usize {
    fn advance_cols(self, byte_ofs: usize, _num_chars: usize) -> Self {
        self + byte_ofs
    }
    fn advance_line(self, byte_ofs: usize) -> Self {
        self + byte_ofs
    }
}

//ip PosnInCharStream for usize
impl PosnInCharStream for usize {
    fn byte_ofs(&self) -> usize {
        *self
    }
}
