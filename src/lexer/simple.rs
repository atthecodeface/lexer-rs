//a Imports
use crate::{
    PosnInCharStream, StreamCharSpan, TokenType,
    TokenTypeError, Lexer, LexerParseResult, LexerOfChar,
};

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
    pub fn parse_char<'a, L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
    {
        let pos = state;
        match ch {
            '\n' => Ok(Some((
                stream.consumed_newline(state, 1),
                Self::Newline(pos),
            ))),
            '(' | '[' | '{' => Ok(Some((
                stream.consumed_chars(state, 1, 1),
                Self::OpenBra(pos, ch),
            ))),
            ')' | ']' | '}' => Ok(Some((
                stream.consumed_chars(state, 1, 1),
                Self::CloseBra(pos, ch),
            ))),
            ch => Ok(Some((
                stream.consume_char(state, ch),
                Self::Char(pos, ch),
            ))),
        }
    }

    //fp parse_comment_line
    pub fn parse_comment_line<'a, L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
    {
        match stream.do_while(state, ch, &|n, ch| {
            ((n < 2) && (ch == '/')) || ((n >= 2) && ch != '\n')
        }) {
            (state, Some((start, _n))) => {
                let span = StreamCharSpan::new(start, state);
                Ok(Some((state, SimpleToken::CommentLine(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_digits
    pub fn parse_digits<'a, L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
    {
        match stream.do_while(state, ch, &|_, ch| ch.is_ascii_digit()) {
            (state, Some((start, _n))) => {
                let span = StreamCharSpan::new(start, state);
                Ok(Some((state, SimpleToken::Digits(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_whitespace
    pub fn parse_whitespace<'a, L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
    {
        match stream.do_while(state, ch, &|_, ch| (ch == ' ' || ch == '\t')) {
            (state, Some((start, _))) => {
                let span = StreamCharSpan::new(start, state);
                Ok(Some((state, SimpleToken::Whitespace(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_id
    pub fn parse_id<'a, L, F1, F2>(
        stream: &L,
        state: L::State,
        ch: char,
        is_id_start: F1,
        is_id: F2,
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
          F1: Fn(char) -> bool,
          F2: Fn(char) -> bool    
    {
        match stream.do_while(state, ch, &|n, ch| {
            (n == 0 && is_id_start(ch)) || ((n > 0) && is_id(ch))
        }) {
            (state, Some((start, _))) => {
                let span = StreamCharSpan::new(start, state);
                Ok(Some((state, SimpleToken::Id(span))))
            }
            (_, None) => Ok(None),
        }
    }

    //fp parse_keyword
    pub fn parse_keyword<L>(
        stream: &L,
        state: L::State,
        _ch: char,
        keywords: &[(&[u8], K)],
    ) -> LexerParseResult<L>
    where L: LexerOfChar,
          L: Lexer<Token = Self, State = P>,
    {
        for (k, v) in keywords {
            if stream.matches_bytes(&state, k) {
                let n = k.len();
                let next_state = stream.consumed_chars(state, n, n);
                return Ok(Some((next_state, SimpleToken::Keyword(state, *v))));
            }
        }
        Ok(None)
    }

    //zz All done
}
