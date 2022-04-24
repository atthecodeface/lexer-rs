//a Imports
use crate::{Pos, Span, TextPos, TextStreamSpan, TokenParseResult, TokenType, TokenTypeError};

//a SimpleToken
//tp SimpleToken
#[derive(Debug, Copy, Clone)]
pub enum SimpleToken<P: TextPos> {
    CommentLine(Span<P>),
    Id(Span<P>),
    Digits(Span<P>),
    Newline(Pos<P>),
    Char(Pos<P>, char),
    OpenBra(Pos<P>, char),
    CloseBra(Pos<P>, char),
    Whitespace(Span<P>),
}

//ip TokenType for SimpleToken
impl<P> TokenType for SimpleToken<P> where P: TextPos {}

//ip SimpleToken
impl<P> SimpleToken<P>
where
    P: TextPos,
{
    //fp parse_char
    pub fn parse_char<E: TokenTypeError<P>>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
    ) -> TokenParseResult<P, Self, E> {
        let pos = stream.pos();
        match ch {
            '\n' => Ok(Some((
                stream.consumed_newline(byte_ofs + 1),
                Self::Newline(pos),
            ))),
            '(' | '[' | '{' => Ok(Some((
                stream.consumed_chars(byte_ofs + 1, 1),
                Self::OpenBra(pos, ch),
            ))),
            ')' | ']' | '}' => Ok(Some((
                stream.consumed_chars(byte_ofs + 1, 1),
                Self::CloseBra(pos, ch),
            ))),
            ch => Ok(Some((
                stream.consume_char(byte_ofs, ch),
                Self::Char(pos, ch),
            ))),
        }
    }
    //fp parse_digits
    pub fn parse_digits<E: TokenTypeError<P>>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
    ) -> TokenParseResult<P, Self, E> {
        match stream.do_while(ch, byte_ofs, &|ch| ch.is_ascii_digit()) {
            (stream, Some((pos, _n))) => {
                let span = Span::new(pos, stream.pos());
                Ok(Some((stream, SimpleToken::Digits(span))))
            }
            (_, None) => Ok(None),
        }
    }
    //fp parse_whitespace
    pub fn parse_whitespace<E: TokenTypeError<P>>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
    ) -> TokenParseResult<P, Self, E> {
        match stream.do_while(ch, byte_ofs, &|ch| (ch == ' ' || ch == '\t')) {
            (stream, Some((pos, _))) => {
                let span = Span::new(pos, stream.pos());
                Ok(Some((stream, SimpleToken::Whitespace(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //zz All done
}
