//a Imports
use crate::{StreamCharPos, StreamCharSpan, PosnInCharStream, TextStreamSpan, TokenParseResult, TokenType, TokenTypeError};

//a SimpleToken
//tt SimpleKeyword
pub trait SimpleKeyword: std::fmt::Debug + Copy + Clone + Sized {}

impl SimpleKeyword for () {}
impl SimpleKeyword for u8 {}
impl SimpleKeyword for u16 {}
impl SimpleKeyword for u32 {}
impl SimpleKeyword for u64 {}
impl SimpleKeyword for usize {}

//tp SimpleToken
#[derive(Debug, Clone, Copy)]
pub enum SimpleToken<P: PosnInCharStream, K: SimpleKeyword> {
    /// Comment is '//' and up to (and not including) a newline
    CommentLine(StreamCharSpan<P>),
    /// ID is an id start and any id following
    Id(StreamCharSpan<P>),
    /// digits is decimal digits
    Digits(StreamCharSpan<P>),
    /// OpenBra is one of ( [ {
    OpenBra(P, char),
    /// CloseBra is one of } ] )
    CloseBra(P, char),
    /// Whitespace is a span of spaces and tabs
    Whitespace(StreamCharSpan<P>),
    /// Keyword is one of the ascii keywords supplied
    Keyword(P, K),
    /// Newline is a Newline
    Newline(P),
    /// Char is an otherwise unknown char
    Char(P, char),
}

//ip TokenType for SimpleToken
impl<P, K> TokenType for SimpleToken<P, K>
where
    P: PosnInCharStream,
    K: SimpleKeyword,
{
}

//ip SimpleToken
impl<P, K> SimpleToken<P, K>
where
    P: PosnInCharStream,
    K: SimpleKeyword,
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

    //fp parse_comment_line
    pub fn parse_comment_line<E: TokenTypeError<P>>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
    ) -> TokenParseResult<P, Self, E> {
        match stream.do_while(ch, byte_ofs, &|n, ch| {
            ((n < 2) && (ch == '/')) || ((n >= 2) && ch != '\n')
        }) {
            (stream, Some((pos, _n))) => {
                let span = StreamCharSpan::new(pos, stream.pos());
                Ok(Some((stream, SimpleToken::CommentLine(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_digits
    pub fn parse_digits<E: TokenTypeError<P>>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
    ) -> TokenParseResult<P, Self, E> {
        match stream.do_while(ch, byte_ofs, &|_, ch| ch.is_ascii_digit()) {
            (stream, Some((pos, _n))) => {
                let span = StreamCharSpan::new(pos, stream.pos());
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
        match stream.do_while(ch, byte_ofs, &|_, ch| (ch == ' ' || ch == '\t')) {
            (stream, Some((pos, _))) => {
                let span = StreamCharSpan::new(pos, stream.pos());
                Ok(Some((stream, SimpleToken::Whitespace(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_id
    pub fn parse_id<E: TokenTypeError<P>, F1: Fn(char) -> bool, F2: Fn(char) -> bool>(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<P>,
        is_id_start: F1,
        is_id: F2,
    ) -> TokenParseResult<P, Self, E> {
        match stream.do_while(ch, byte_ofs, &|n, ch| {
            (n == 0 && is_id_start(ch)) || ((n > 0) && is_id(ch))
        }) {
            (stream, Some((pos, _))) => {
                let span = StreamCharSpan::new(pos, stream.pos());
                Ok(Some((stream, SimpleToken::Id(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_keyword
    pub fn parse_keyword<'a, E: TokenTypeError<P>>(
        _ch: char,
        byte_ofs: usize,
        mut stream: TextStreamSpan<'a, P>,
        keywords: &[(&[u8], K)],
    ) -> TokenParseResult<'a, P, Self, E> {
        for (k, v) in keywords {
            if stream.matches_bytes(byte_ofs, k) {
                let n = k.len();
                let pos = stream.pos();
                stream = stream.consumed_chars(byte_ofs + n, n);
                return Ok(Some((stream, SimpleToken::Keyword(pos, *v))));
            }
        }
        Ok(None)
    }

    //zz All done
}
