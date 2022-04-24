//a Imports
use std::ops::Range;

use crate::{Pos, Span, TextPos};

//a External traits
//tt TokenType
/// The trait required of a token
pub trait TokenType: Sized + std::fmt::Debug + Copy {}

//tt TokenTypeError
/// A trait required of an error - a char that does not match any
/// token parser rust return an error, and this trait requires that
/// such an error be provided
pub trait TokenTypeError<P: TextPos>: Sized + std::error::Error {
    fn failed_to_parse(ch: char, stream: TextStreamSpan<P>) -> Self;
}

//tp TokenParseResult
/// The result of attempting to parse a token in a stream
///
/// The result can be an Err, in which case the stream cannot be parsed as tokens
///
/// If the result is Ok(None), then a parser has failed to parse the
/// next token in the stream, and another parser can be attempted
///
/// If the result is Some((stream, token)) then the token has been
/// parsed and the stream has been moved on beyond that token
// pub type TokenParseResult<'a, P : TextPos, T : TokenType, E: TokenTypeError<P>> = Result<Option<(TextStreamSpan<'a, P>, T)>, E>;
pub type TokenParseResult<'a, P, T, E> = Result<Option<(TextStreamSpan<'a, P>, T)>, E>;

//tp TokenParser
/// The type of ...
// pub type TokenParser<'a, P : TextPos, T : TokenType, E: TokenTypeError<P>> = dyn Fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;
// pub type TokenParser<'a, P,  T, E> = dyn Fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;
// pub type TokenParser<'a, P,  T, E> = fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;
// pub type TokenParser<P, T, E> =
//    for<'a> fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;
pub type TokenParser<'a, P, T, E> =
    fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;

//tp TokenParserError
#[derive(Debug)]
pub struct TokenParseError<P>
where
    P: TextPos,
{
    s: String,
    pos: Pos<P>,
}

impl<P: TextPos> std::error::Error for TokenParseError<P> {}
impl<P> TokenTypeError<P> for TokenParseError<P>
where
    P: TextPos,
{
    fn failed_to_parse(ch: char, stream: TextStreamSpan<P>) -> Self {
        let s = format!("Failed to parse: unexpected char '{}'", ch);
        let pos = stream.pos();
        Self { s, pos }
    }
}
impl<P> std::fmt::Display for TokenParseError<P>
where
    P: TextPos,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} at {}", self.s, self.pos)
    }
}

//a TextStream, TextStreamSpan
//tp TextStream
/// A wrapper around a str slice that allows simple access and
/// matching
#[derive(Debug)]
pub struct TextStream<'a> {
    text: &'a str,
}

//ip TextStream
impl<'a> TextStream<'a> {
    //fp new
    /// Create a new [TextStream] by borrowing a [str]
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    //mp as_span
    /// Borrow the full [TextStreamSpan] of the [TextStream]
    pub fn as_span<P: TextPos>(&self) -> TextStreamSpan<P> {
        TextStreamSpan::new(self)
    }

    //mp get_text_span
    /// Get the text of a [Span] provided by the parsers
    ///
    /// Safety : The Span has been provided by a parser and so the
    /// byte offsets are indeed utf8 character boundaries
    pub fn get_text_span<P: TextPos>(&self, span: Span<P>) -> &str {
        unsafe { self.get_text(span.byte_range()) }
    }

    //mp get_text
    /// Get the text between a start and end byte offset
    ///
    /// Safety : The byte offsets must correspond to utf8 character points
    unsafe fn get_text(&self, range: Range<usize>) -> &str {
        self.text.get_unchecked(range)
    }

    //mp peek
    /// Get the utf8 chararacter at the byte offset, or None at the end of a string
    fn peek(&self, byte_ofs: usize) -> Option<char> {
        if byte_ofs >= self.text.len() {
            None
        } else {
            // Safety : byte_ofs is maintained as a utf8 character
            // point boundary within or at the end of the str
            let text = unsafe { self.text.get_unchecked(byte_ofs..self.text.len()) };
            text.chars().next()
        }
    }

    //mp as_bytes
    /// Borrow some bytes of the stream from an offset
    ///
    /// Return None if the bytes are out of range
    pub fn as_bytes(&self, ofs: usize, n: usize) -> &[u8] {
        assert!(ofs + n <= self.text.len());
        &self.text.as_bytes()[ofs..ofs + n]
    }

    //zz All done
}

//tp TextStreamSpan
#[derive(Debug, Copy, Clone)]
pub struct TextStreamSpan<'a, P>
where
    P: TextPos,
{
    stream: &'a TextStream<'a>,
    end: usize,
    pos: Pos<P>,
}

//ip TextStreamSpan
impl<'a, P> TextStreamSpan<'a, P>
where
    P: TextPos,
{
    //fp new
    /// Create a new [TextStreamSpan] of the whole of a [TextStream]
    fn new(stream: &'a TextStream<'a>) -> Self {
        Self {
            stream,
            end: stream.text.len(),
            pos: Pos::default(),
        }
    }

    //mp get_text
    /// Get the text corresponding to this span
    pub fn get_text(&self) -> &str {
        // Safety : byte_ofs is maintained as a utf8 character
        // point boundary within or at the end of the str
        unsafe { self.stream.get_text(self.pos.byte_ofs()..self.end) }
    }

    //mp pos
    pub fn pos(&self) -> Pos<P> {
        self.pos
    }

    //mp peek
    /// Peek at the next character
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.stream.peek(self.pos.byte_ofs())
    }

    //mp peek_at
    /// Peek at the a byte offset ahead
    #[inline]
    pub fn peek_at(&self, byte_ofs: usize) -> Option<char> {
        self.stream.peek(byte_ofs)
    }

    //mp matches_bytes
    /// Match the text at the offset with a str
    pub fn matches_bytes(&self, byte_ofs:usize, s: &[u8]) -> bool {
        let n = s.len();
        if byte_ofs + n > self.end {
            false
        } else {
            s == self.stream.as_bytes(byte_ofs, n)
        }
    }

    //mp matches
    /// Match the text at the offset with a str
    pub fn matches(&self, byte_ofs:usize, s: &str) -> bool {
        self.matches_bytes(byte_ofs, s.as_bytes())
    }

    //cp consume_char
    /// Become the span after consuming a particular char
    pub fn consume_char(mut self, byte_ofs: usize, ch: char) -> Self {
        if ch == '\n' {
            self.pos = self.pos.advance_line(byte_ofs + 1);
        } else {
            self.pos = self.pos.advance_cols(byte_ofs + ch.len_utf8(), 1);
        }
        self
    }

    //cp consume_ascii_str
    /// Become the span after consuming a particular ascii string without newlines
    pub fn consume_ascii_str(mut self, byte_ofs: usize, s: &str) -> Self {
        let n = s.len();
        self.pos = self.pos.advance_cols(byte_ofs + n, n);
        self
    }

    //cp consumed_chars
    /// Become the span after consuming a particular string of known character length
    pub fn consumed_chars(mut self, end_ofs: usize, num_chars: usize) -> Self {
        self.pos = self.pos.advance_cols(end_ofs, num_chars);
        self
    }

    //cp consumed_newline
    /// Become the stream span after a newline
    pub fn consumed_newline(mut self, end_ofs: usize) -> Self {
        self.pos = self.pos.advance_line(end_ofs);
        self
    }

    //mp count_matching
    pub fn do_while<F: Fn(usize, char) -> bool>(
        mut self,
        ch: char,
        byte_ofs: usize,
        f: &F,
    ) -> (Self, Option<(Pos<P>, usize)>) {
        if !f(0, ch) {
            return (self, None);
        }
        let mut n = 1;
        let mut ofs = byte_ofs + ch.len_utf8();
        while let Some(ch) = self.peek_at(ofs) {
            if !f(n, ch) {
                break;
            }
            n += 1;
            ofs += ch.len_utf8();
        }
        let pos = self.pos();
        self = self.consumed_chars(ofs, n);
        (self, Some((pos, n)))
    }

    //mp parse
    pub fn parse<T, E>(self, parsers: &'a [TokenParser<'a, P, T, E>]) -> TokenParseResult<'a, P, T, E>
    where
        T: TokenType,
        E: TokenTypeError<P>,
    {
        if let Some(ch) = self.peek() {
            for p in parsers {
                let result = p(ch, self.pos.byte_ofs(), self)?;
                if result.is_some() {
                    return Ok(result);
                }
            }
            return Err(E::failed_to_parse(ch, self));
        }
        Ok(None)
    }

    //mp iter_tokens
    pub fn iter_tokens<T, E>(
        self,
        parsers: &'a [TokenParser<'a, P, T, E>],
    ) -> TextStreamSpanIterator<'a, P, T, E>
    where
        T: TokenType,
        E: TokenTypeError<P>,
    {
        TextStreamSpanIterator::new(self, parsers)
    }
}

pub struct TextStreamSpanIterator<'a, P, T, E>
where
    P: TextPos,
    T: TokenType,
    E: TokenTypeError<P>,
{
    stream: TextStreamSpan<'a, P>,
    parsers: &'a [TokenParser<'a, P, T, E>],
}

impl<'a, P, T, E> TextStreamSpanIterator<'a, P, T, E>
where
    P: TextPos,
    T: TokenType,
    E: TokenTypeError<P>,
{
    pub fn new(stream: TextStreamSpan<'a, P>, parsers: &'a [TokenParser<'a, P, T, E>]) -> Self {
        Self { stream, parsers }
    }
}

impl<'a, P, T, E> Iterator for TextStreamSpanIterator<'a, P, T, E>
where
    P: TextPos,
    T: TokenType,
    E: TokenTypeError<P>,
{
    type Item = Result<T, E>;
    fn next(&mut self) -> Option<Result<T, E>> {
        match self.stream.parse(self.parsers) {
            Err(e) => Some(Err(e)),
            Ok(Some((stream, token))) => {
                self.stream = stream;
                Some(Ok(token))
            }
            _ => None,
        }
    }
}
