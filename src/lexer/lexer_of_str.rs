//a Imports
use std::marker::PhantomData;

use crate::BoxDynLexerParseFn;
use crate::{Lexer, LexerError, LexerOfChar, LexerParseResult};
use crate::{ParserIterator, PosnInCharStream, StreamCharSpan};

//a LexerOfStr
//tp LexerOfStr
/// A [Lexer] of a [str], using an arbitrary stream position type,
/// lexer token, and lexer error.
///
/// This provides implementations of [Lexer] and [LexerOfChar].
///
/// The [Lexer] implementation means that a [LexerOfStr] has a 'parse'
/// method that can be invoked to parse a single token at a position
/// within the [str], and another 'iter' method that can be invoked to
/// generate an iterator that returns all the tokens in the [str]
///
/// If the iterator or parser return an Err, the that error is of the
/// generic type 'E' supplied to the [LexerOfStr] which must implement
/// [LexerError] of the generic position 'P' - so a failure to parse a
/// character in the string can be indicated at a particular location
/// (byte offset, with line number and column potentially).
///
/// The actual parsing of tokens is supported through the [Lexer]
/// trait for both the 'parser' and 'iter' trait methods using a
/// &[BoxDynLexerParseFn]. These must be boxed functions with the signature
/// like:
///
/// ```ignore
///    fn parse(stream: &LexerOfStr<P, T, E>, pos:P, ch:char) ->
///               LexerParseResult<P, T, E>
/// ```
///
/// where
///
/// ```ignore
///    LexerParseResult<P, T, E> = Result<Option<P, T>, E>
/// ```
///
/// See the [Lexer] trait for more details on these parse functions
///
/// The [LexerOfStr] also provides a [LexerOfChar] implementation,
/// which provides methods that are can be used by the parse functions.
///
/// This provides methods to match strings, get 
///
// Cannot derive either Copy or Clone without that putting the same bound on T and E
#[derive(Debug)]
pub struct LexerOfStr<'a, P, T, E>
where
    P: PosnInCharStream,
{
    text: &'a str,
    end: usize,
    _phantom_posn: PhantomData<&'a P>,
    _phantom_token: PhantomData<&'a T>,
    _phantom_error: PhantomData<&'a E>,
}

//ip Copy for LexerOfStr<'a, P, T, E>
impl<'a, P, T, E> Copy for LexerOfStr<'a, P, T, E> where P: PosnInCharStream {}

//ip Clone for LexerOfStr<'a, P, T, E>
impl<'a, P, T, E> Clone for LexerOfStr<'a, P, T, E>
where
    P: PosnInCharStream,
{
    fn clone(&self) -> Self {
        *self
    }
}

//ip LexerOfStr
impl<'a, P, T, E> LexerOfStr<'a, P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    //fp new
    /// Create a new [TextStream] by borrowing a [str]
    pub fn new(text: &'a str) -> Self {
        let end = text.as_bytes().len();
        Self {
            text,
            end,
            _phantom_posn: PhantomData,
            _phantom_token: PhantomData,
            _phantom_error: PhantomData,
        }
    }

    //mp peek_at_offset
    /// Get the utf8 chararacter at the byte offset, or None at the end of a string
    unsafe fn peek_at_offset(&self, byte_ofs: usize) -> Option<char> {
        if byte_ofs >= self.end {
            None
        } else {
            let text = self.text.get_unchecked(byte_ofs..self.end);
            text.chars().next()
        }
    }

    //mp remaining_text
    /// Get the remaining text from a position
    fn remaining_text(&self, p: &P) -> &str {
        // # Safety
        //
        // Safe if p is a valid Posn as then it must be a utf8
        // character boundary
        unsafe { self.text.get_unchecked(p.byte_ofs()..self.end) }
    }
}

//a Impl Lexer, LexerOfChar
//ip Lexer for LexerOfStr
impl<'a, P, T, E> Lexer for LexerOfStr<'a, P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    type Token = T;
    type Error = E;
    type State = P;

    //mp parse
    fn parse<'iter>(
        &'iter self,
        state: Self::State,
        parsers: &[BoxDynLexerParseFn<'iter, Self>],
    ) -> LexerParseResult<Self::State, Self::Token, Self::Error> {
        if let Some(ch) = self.peek_at(&state) {
            for p in parsers {
                let result = p(&self, state, ch)?;
                if result.is_some() {
                    return Ok(result);
                }
            }
            return Err(E::failed_to_parse(state, ch));
        }
        Ok(None)
    }

    //mp iter
    fn iter<'iter>(
        &'iter self,
        parsers: &'iter [BoxDynLexerParseFn<'iter, Self>],
    ) -> Box<dyn Iterator<Item = Result<T, E>> + 'iter> {
        let state = Default::default();
        Box::new(ParserIterator::new(self, state, parsers))
    }
}

//ip LexerOfChar for LexerOfStr
impl<'a, P, T, E> LexerOfChar<P> for LexerOfStr<'a, P, T, E>
where
    P: PosnInCharStream,
    T: Sized + std::fmt::Debug + Copy,
    E: LexerError<P>,
{
    //mp range_as_bytes
    /// Borrow some bytes of the stream from an offset
    ///
    /// Return None if the bytes are out of range
    fn range_as_bytes(&self, ofs: usize, n: usize) -> &[u8] {
        assert!(ofs + n <= self.end);
        &self.text.as_bytes()[ofs..ofs + n]
    }

    //mp get_text_span
    /// Get the text of a [StreamCharSpan] provided by a parser
    ///
    /// # Safety
    ///
    /// The [StreamCharSpan] must have been provided by a parser and
    /// so the byte offsets are indeed utf8 character boundaries
    fn get_text_span(&self, span: &StreamCharSpan<P>) -> &str {
        unsafe { self.text.get_unchecked(span.byte_range()) }
    }

    //mp get_text
    /// Get the text between two [StreamCharPos] provided by a parser
    ///
    /// # Safety
    ///
    /// The [StreamCharPos] must have been provided by a parser and
    /// so the byte offsets are indeed utf8 character boundaries
    fn get_text(&self, start: P, end: P) -> &str {
        unsafe { self.text.get_unchecked(start.byte_ofs()..end.byte_ofs()) }
    }

    //mp peek_at
    /// Get the utf8 chararacter at the byte offset, or None at the end of a string
    ///
    /// # Safety
    ///
    /// 'state' is maintained as a utf8 character point boundary
    /// within or at the end of the 'str' borrowed by [Self]
    fn peek_at(&self, state: &P) -> Option<char> {
        unsafe { self.peek_at_offset(state.byte_ofs()) }
    }

    //mp matches_bytes
    /// Match the text at the offset with a str
    fn matches_bytes(&self, state: &P, s: &[u8]) -> bool {
        let n = s.len();
        let byte_ofs = state.byte_ofs();
        if byte_ofs + n > self.end {
            false
        } else {
            s == self.range_as_bytes(byte_ofs, n)
        }
    }

    //mp matches_str
    /// Match the text at the offset with a str
    fn matches_str(&self, pos: &P, pat: &str) -> bool {
        self.remaining_text(pos).starts_with(pat)
    }

    //mp matches - awaiting Pattern stabilization
    // Match the text at the offset with a str
    // fn matches<'call, Pat:std::str::pattern::Pattern<'call>>(&self, pos: &P, pat: Pat) -> bool {
    // self.remaining_text(pos).starts_with(pat)
    // }

    //cp consumed
    fn consumed(&self, mut state: P, mut n: usize) -> P {
        for ch in self.remaining_text(&state).chars() {
            if n == 0 {
                break;
            }
            if ch == '\n' {
                state = state.advance_line(1)
            } else {
                state = state.advance_cols(ch.len_utf8(), 1)
            }
            n -= 1;
        }
        state
    }
    
    //mp do_while
    fn do_while<F: Fn(usize, char) -> bool>(
        &self,
        mut state: P,
        ch: char,
        f: &F,
    ) -> (P, Option<(P, usize)>) {
        if !f(0, ch) {
            return (state, None);
        }
        let start = state;
        let mut n = 1;
        let mut ofs = state.byte_ofs() + ch.len_utf8();
        // # Safety
        //
        // 'ofs' is maintained as a utf8 character point boundary
        // within or at the end of the 'str' borrowed by [Self]
        while let Some(ch) = unsafe { self.peek_at_offset(ofs) } {
            if !f(n, ch) {
                break;
            }
            n += 1;
            ofs += ch.len_utf8();
        }
        // Does not work if newlines are involved
        state = unsafe { self.consumed_chars(state, ofs - start.byte_ofs(), n) };
        (state, Some((start, n)))
    }
}
