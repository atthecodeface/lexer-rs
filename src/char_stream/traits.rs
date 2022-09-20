//a Imports
use crate::{PosnInCharStream, StreamCharSpan};

//tt CharStream
/// The [CharStream] trait allows a stream of [char] to provide extraa methods
///
/// Requires P : PosnInCharStream
pub trait CharStream<P> {
    /// Steps along the stream starting at the provided state (and
    /// character) while the provided function returns true; the
    /// function is provided with the index and character (starting at
    /// 0 / ch), and it returns true if the token continues, otherwise
    /// false
    ///
    /// If the first invocation of 'f' returns false then the token is
    /// said to not match, and 'do_while' returns the stream state and Ok(None).
    ///
    /// If the first N (more than zero) invocations match then the
    /// result is the stream state after the matched characters, and
    /// Some(initial state, N)
    ///
    /// This can be used to match whitespace (where N is probably
    /// discarded), or user 'id' values in a language. The text can be
    /// retrieved with the 'get_text' method
    fn do_while<F: Fn(usize, char) -> bool>(
        &self,
        state: P,
        ch: char,
        f: &F,
    ) -> (P, Option<(P, usize)>);

    /// Retrieve a range of bytes from the stream
    fn range_as_bytes(&self, ofs: usize, n: usize) -> &[u8];

    /// Return true if the content of the stream at 'state' matches
    /// the byte slice
    fn matches_bytes(&self, state: &P, s: &[u8]) -> bool;

    /// Get the text between the start of a span (inclusive) and the
    /// end of the span (exclusive).
    fn get_text_span(&self, span: &StreamCharSpan<P>) -> &str
    where
        P: PosnInCharStream;

    /// Get the text between the start (inclusive) and the
    /// end (exclusive).
    fn get_text(&self, start: P, end: P) -> &str;

    // Return true if the text at 'pos' matches the string
    //
    // Waiting for pattern stabiliztion
    // fn matches<'call, P:std::str::pattern::Pattern<'call>>(&self, pos: &P, pat: P) -> bool;

    /// Match the text at the offset with a str; return true if it matches, else false
    fn matches_str(&self, pos: &P, pat: &str) -> bool;

    /// Peek at the next character in the stream, returning None if
    /// the state is the end of the stream
    fn peek_at(&self, state: &P) -> Option<char>;

    //cp consumed
    /// Move the stream state forward by the specified number of characters
    fn consumed(&self, state: P, num_chars: usize) -> P;

    //cp consumed_char
    /// Get a stream state after consuming the specified (non-newline) character at its current state
    fn consumed_char(&self, state: P, ch: char) -> P where P : PosnInCharStream  {
      if ch == '\n' {
          state.advance_line(1)
       } else {
          state.advance_cols(ch.len_utf8(), 1)
       }
    }        

    //cp consumed_newline
    /// Get a stream state after consuming a newline at its current state
    unsafe fn consumed_newline(&self, state: P, num_bytes: usize) -> P where P : PosnInCharStream {
        state.advance_line(num_bytes)    
    }

    //cp consumed_ascii_str
    /// Get a stream state after consuming the specified (non-newline) character at its current state
    /// Become the span after consuming a particular ascii string without newlines
    unsafe fn consumed_ascii_str(&self, state: P, s: &str) -> P where P : PosnInCharStream {
        let n = s.len();
        state.advance_cols( n, n)
    }

    //cp consumed_chars
    /// Become the span after consuming a particular string of known character length
    unsafe fn consumed_chars(&self, state: P, num_bytes: usize, num_chars: usize) -> P where P : PosnInCharStream {
         state.advance_cols(num_bytes, num_chars)
    }

    //mp commit_consumed
    /// Invoked by the Lexer to indicate that the stream has been
    /// consumed up to a certain point, and that (for parsing) no
    /// state earlier in the stream will be requested in the future
    ///
    /// A truly streaming source can drop earlier data in the stream
    /// if this fits the application
    fn commit_consumed(&self, _up_to: &P) {}
}

