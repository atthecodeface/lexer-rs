//a Imports
use std::marker::PhantomData;

use crate::BoxDynLexerParseFn;
use crate::{Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult};
use crate::{ParserIterator, PosnInCharStream, StreamCharSpan};

//a Impl Lexer
//tp LexerOfStr
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

    //mp iter_tokens
    pub fn iter_tokens<'iter>(
        &'iter self,
        parsers: &'iter [BoxDynLexerParseFn<'iter, Self>],
    ) -> ParserIterator<'iter, Self> {
        let state = P::default();
        ParserIterator::new(self, state, parsers)
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
}

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
    fn iter<'iter>(
        &'iter self,
        parsers: &'iter [BoxDynLexerParseFn<'iter, Self>],
    ) -> Box<dyn Iterator<Item = Result<T, E>> + 'iter> {
        let state = Default::default();
        Box::new(ParserIterator::new(self, state, parsers))
    }
}

//ip LexerOfChar for LexerOfStr
impl<'a, P, T, E> LexerOfChar for LexerOfStr<'a, P, T, E>
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

    //cp consume_char
    /// Become the span after consuming a particular char
    fn consume_char(&self, state: P, ch: char) -> P {
        if ch == '\n' {
            state.advance_line(1)
        } else {
            state.advance_cols(ch.len_utf8(), 1)
        }
    }

    //cp consume_ascii_str
    /// Become the span after consuming a particular ascii string without newlines
    fn consume_ascii_str(&self, state: P, s: &str) -> P {
        let n = s.len();
        state.advance_cols(n, n)
    }

    //cp consumed_chars
    /// Become the span after consuming a particular string of known character length
    fn consumed_chars(&self, state: P, num_bytes: usize, num_chars: usize) -> P {
        state.advance_cols(num_bytes, num_chars)
    }

    //cp consumed_newline
    /// Become the stream span after a newline
    fn consumed_newline(&self, state: P, num_bytes: usize) -> P {
        state.advance_line(num_bytes)
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

    //mp matches
    /// Match the text at the offset with a str
    fn matches(&self, state: &P, s: &str) -> bool {
        self.matches_bytes(state, s.as_bytes())
    }

    //p do_while
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
        state = self.consumed_chars(state, ofs - start.byte_ofs(), n);
        (state, Some((start, n)))
    }
}