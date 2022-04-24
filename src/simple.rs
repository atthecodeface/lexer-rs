//a Imports
use crate::{TextPos, Pos, Span, TokenType, TokenTypeError, TokenParseResult, TextStreamSpan};

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
impl <P> TokenType for SimpleToken<P>
where P:TextPos {}

//ip SimpleToken
impl <P> SimpleToken<P>
where P:TextPos {
    //fp parse_char
    pub fn parse_char<'a, E:TokenTypeError<P>> (ch: char, byte_ofs:usize, stream : TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, Self, E> {
        let pos = stream.pos();
        match ch {
            '\n' => {
                Ok(Some((stream.consumed_newline(byte_ofs+1), Self::Newline(pos))))
            }
            '(' | '[' | '{' => {
                Ok(Some((stream.consumed_chars(byte_ofs+1, 1), Self::OpenBra(pos, ch))))
            }
            ')' | ']' | '}' => {
                Ok(Some((stream.consumed_chars(byte_ofs+1, 1), Self::CloseBra(pos, ch))))
            }
/*            ' ' | '\t' => {
            Ok(Some((Token::Whitespace(pos), stream.consumed_chars(byte_ofs+1, 1))))
            }
*/
            ch => {
                Ok(Some((stream.consume_char(byte_ofs, ch), Self::Char(pos, ch))))
            }
        }
    }
    //fp parse_digits
    pub fn parse_digits<'a, E:TokenTypeError<P>> (ch: char, byte_ofs:usize, stream : TextStreamSpan<'a, P>) -> TokenParseResult<'a, P, Self, E> {
        if !ch.is_ascii_digit() {
            return Ok(None);
        }
        let mut n = 1;
        loop {
            if let Some(ch) = stream.peek_at(n) {
                if !ch.is_ascii_digit() {
                    break;
                }
                n += 1;
            } else {
                break;
            }
        }
        let pos = stream.pos();
        let stream = stream.consumed_chars( byte_ofs+n, n);
        let span = Span::new(pos, stream.pos());
        Ok(Some((stream, SimpleToken::Digits(span))))
    }
    //zz All done
}
