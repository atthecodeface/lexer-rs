//a Imports
use std::ops::Range;

use crate::{Pos, Span, TextPos};

//a External traits
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
///
/// P : TextPos
///
/// T : TokenType
///
/// E: TokenTypeError<P>
pub type TokenParseResult<'a, P, T, E> = Result<Option<(TextStreamSpan<'a, P>, T)>, E>;

//tp TokenParser
/// A function that maps a character, usize byte offset within the stream, and a stream to a token
///
/// Parsers return Ok(Some(T)) if it parses a token of type T; Ok(None) if they fail to parse; Err(TokenTypeError) if they
///
/// P : TextPos
///
/// T : TokenType
///
/// E: TokenTypeError<P>
pub type TokenParser<'a, P, T, E> =
    fn(char, usize, TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, T, E>;

//tp TokenParseError
/// A simple implementation of a type supporting TokenTypeError
///
/// An error in parsing a token - most often an 'unrecognized character'
///
/// P : TextPos
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenParseError<P>
where
    P: TextPos,
{
    s: String,
    pos: Pos<P>,
}

//ip Error for TokenParseError
impl<P> std::error::Error for TokenParseError<P>
where
    P: TextPos + std::fmt::Display,
{}

//ip TokenTypeError for TokenParseError
impl<P> TokenTypeError<P> for TokenParseError<P>
where
    P: TextPos + std::fmt::Display,
{
    fn failed_to_parse(ch: char, stream: TextStreamSpan<P>) -> Self {
        let s = format!("Failed to parse: unexpected char '{}'", ch);
        let pos = stream.pos();
        Self { s, pos }
    }
}

//ip Display for TokenParseError
impl<P> std::fmt::Display for TokenParseError<P>
where
    P: TextPos + std::fmt::Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} at {}", self.s, self.pos)
    }
}

//a TextStreamSpan
//tp TextStreamSpan
#[derive(Debug, Copy, Clone)]
pub struct TextStreamSpan<'a, P>
where
    P: TextPos,
{
    text: &'a str,
    end: usize,
    pos: Pos<P>,
}

//ip TextStreamSpan
impl<'a, P> TextStreamSpan<'a, P>
where
    P: TextPos,
{
    //fp new
    /// Create a new [TextStream] by borrowing a [str]
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            end: text.len(),
            pos: Pos::default(),
        }
    }

    //ap pos
    pub fn pos(&self) -> Pos<P> {
        self.pos
    }

    //mp as_bytes
    /// Borrow some bytes of the stream from an offset
    ///
    /// Return None if the bytes are out of range
    pub fn as_bytes(&self, ofs: usize, n: usize) -> &[u8] {
        assert!(ofs + n <= self.text.len());
        &self.text.as_bytes()[ofs..ofs + n]
    }

    //mp get_text_of_range
    /// Get the text between a start and end byte offset
    ///
    /// Safety : The byte offsets must correspond to utf8 character points
    pub unsafe fn get_text_of_range(&self, range: Range<usize>) -> &str {
        self.text.get_unchecked(range)
    }

    //mp get_text
    /// Get all the text of this stream - from pos to end
    ///
    /// Safety : The Span has been provided by a parser and so the
    /// byte offsets are indeed utf8 character boundaries
    pub fn get_text(&self) -> &str {
        unsafe { self.get_text_of_range(self.pos.byte_ofs()..self.end) }
    }

    //mp get_text_span
    /// Get the text of a [Span] provided by the parsers
    ///
    /// Safety : The Span has been provided by a parser and so the
    /// byte offsets are indeed utf8 character boundaries
    pub fn get_text_span(&self, span: Span<P>) -> &str {
        unsafe { self.get_text_of_range(span.byte_range()) }
    }

    //mp peek_at
    /// Get the utf8 chararacter at the byte offset, or None at the end of a string
    fn peek_at(&self, byte_ofs: usize) -> Option<char> {
        if byte_ofs >= self.text.len() {
            None
        } else {
            // Safety : byte_ofs is maintained as a utf8 character
            // point boundary within or at the end of the str
            let text = unsafe { self.text.get_unchecked(byte_ofs..self.text.len()) };
            text.chars().next()
        }
    }

    //mp peek
    /// Peek at the next character
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.peek_at(self.pos.byte_ofs())
    }

    //mp matches_bytes
    /// Match the text at the offset with a str
    pub fn matches_bytes(&self, byte_ofs: usize, s: &[u8]) -> bool {
        let n = s.len();
        if byte_ofs + n > self.end {
            false
        } else {
            s == self.as_bytes(byte_ofs, n)
        }
    }

    //mp matches
    /// Match the text at the offset with a str
    pub fn matches(&self, byte_ofs: usize, s: &str) -> bool {
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

    //mp do_while
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
    /// Parse the next token in a stream, returning an error if no parser matches the data
    ///
    /// At the end of the stream Ok(None) is returned
    pub fn parse<T, E>(
        self,
        parsers: &'a [TokenParser<'a, P, T, E>],
    ) -> TokenParseResult<'a, P, T, E>
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

//tp TextStreamSpanIterator
/// An iterator over a TextStream presenting the parsed Tokens from it
pub struct TextStreamSpanIterator<'a, P, T, E>
where
    P: TextPos,
    T: TokenType,
    E: TokenTypeError<P>,
{
    stream: TextStreamSpan<'a, P>,
    parsers: &'a [TokenParser<'a, P, T, E>],
}

//ip TextStreamSpanIterator
impl<'a, P, T, E> TextStreamSpanIterator<'a, P, T, E>
where
    P: TextPos,
    T: TokenType,
    E: TokenTypeError<P>,
{
    /// Create a new token stream iterator to parse a string and deliver tokens
    pub fn new(stream: TextStreamSpan<'a, P>, parsers: &'a [TokenParser<'a, P, T, E>]) -> Self {
        Self { stream, parsers }
    }
}

//ip Iterator for TextStreamSpanIterator
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
