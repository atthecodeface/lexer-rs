//a Imports
use std::marker::PhantomData;

use crate::{FmtContext, PosnInCharStream};
use crate::{LexerError, LexerOfStr};

//a LexerOfStrinng
//tp LexerOfString
/// This provides a type that wraps an allocated [String], and which
/// tracks the lines within the string. It then provides a method to
/// create a [LexerOfStr] that borrows the text, and which can the be
/// used as a [Lexer].
///
/// This type also implements the [FmtContext] trait, which allows for
/// pretty-printing the text between certain lines, to highlight certain
/// characters or regions of the text.
///
/// Should the [Lexer] return an error while parsing, then the
/// [FmtContext] implementation can be used to generate a very useful
/// error message with the context of the error.
///
/// This also applies should the [Lexer] output be used in a further
/// grammar - the methods to pretty-print the text require just start
/// and end positions of the position type, which must support
/// [PosnInCharStream].
///
/// The [LexerOfString] may be used more than once, replacing its text
/// if necessary before a new parse is initiated. The [String] can
/// also be retrieved, which is useful for droppign the
/// [LexerOfString] but retaining the text for future parse or
/// compilation stages
///
#[derive(Debug)]
pub struct LexerOfString<P, T, E>
where
    P: PosnInCharStream,
{
    text: String,
    line_start_ncolumns: Vec<(P, usize)>,
    _phantom_token: PhantomData<T>,
    _phantom_error: PhantomData<E>,
}

//ip LexerOfString
impl<P, T, E> Default for LexerOfString<P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    fn default() -> Self {
        Self {
            text: String::new(),
            line_start_ncolumns: Vec::new(),
            _phantom_token: PhantomData,
            _phantom_error: PhantomData,
        }
    }
}

//ip LexerOfString
impl<P, T, E> LexerOfString<P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    //cp set_text
    /// Set the text
    pub fn set_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self.find_line_starts();
        self
    }

    //mp take_text
    /// Take the text as a [String] out of the [LexerOfString]
    pub fn take_text(&mut self) -> String {
        self.line_start_ncolumns.clear();
        std::mem::take(&mut self.text)
    }

    //mp text
    /// Get a [str] reference to the text content
    pub fn text(&self) -> &str {
        &self.text
    }

    //mp lexer
    /// Create a [LexerOfStr] that will parse the text
    pub fn lexer(&self) -> LexerOfStr<P, T, E> {
        LexerOfStr::new(&self.text)
    }

    //mi find_line_starts
    /// Finds the start byte offset and number of columns for all the
    /// lines in the text
    fn find_line_starts(&mut self) {
        let mut line_start_ncolumns = Vec::new();
        let mut s: &str = &self.text;
        let mut pos = P::default();
        line_start_ncolumns.push((pos, 0)); // Line '0'
        while let Some((line, next_line)) = s.split_once('\n') {
            let ncolumns = line.chars().count();
            line_start_ncolumns.push((pos, ncolumns));
            pos = pos.advance_line(line.len() + 1);
            s = next_line;
        }
        let ncolumns = s.chars().count();
        line_start_ncolumns.push((pos, ncolumns));
        self.line_start_ncolumns = line_start_ncolumns;
    }
}

//a Impl FmtContext
//ip FmtContext for LexerOfString
impl<P, T, E> FmtContext<P> for LexerOfString<P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    fn line_length(&self, line: usize) -> usize {
        self.line_start_ncolumns[line].1
    }

    fn fmt_line(&self, f: &mut dyn std::fmt::Write, line: usize) -> std::fmt::Result {
        let s = &self.text[self.line_start_ncolumns[line].0.byte_ofs()..];
        let s = s.split_once('\n').map(|(s, _)| s).unwrap_or(s);
        write!(f, "{}", s)
    }
}
