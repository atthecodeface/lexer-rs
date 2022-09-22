//a Imports
use lexer::PosnInCharStream;
use lexer::SimpleParseError;
use lexer::{CharStream, FmtContext, Lexer, LexerOfStr, LexerOfString, LexerParseResult};
use lexer::{LineColumn, StreamCharPos, StreamCharSpan};

//a SimpleToken
//tp SimpleToken
#[derive(Debug, Clone, Copy)]
pub enum SimpleToken<P, K>
where
    P: PosnInCharStream,
    K: std::fmt::Debug + Copy + Clone + Sized,
{
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

//ip SimpleToken
impl<P, K> SimpleToken<P, K>
where
    P: PosnInCharStream,
    K: std::fmt::Debug + Copy + Clone + Sized,
{
    //fp parse_char
    pub fn parse_char<L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
        L: Lexer<Token = Self, State = P>,
    {
        let pos = state;
        match ch {
            '\n' => Ok(Some((stream.consumed(state, 1), Self::Newline(pos)))),
            '(' | '[' | '{' => Ok(Some((stream.consumed(state, 1), Self::OpenBra(pos, ch)))),
            ')' | ']' | '}' => Ok(Some((stream.consumed(state, 1), Self::CloseBra(pos, ch)))),
            ch => Ok(Some((stream.consumed_char(state, ch), Self::Char(pos, ch)))),
        }
    }

    //fp parse_comment_line
    pub fn parse_comment_line<L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
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
    pub fn parse_digits<L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
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
    pub fn parse_whitespace<L>(
        stream: &L,
        state: L::State,
        ch: char,
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
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
    pub fn parse_id<L, F1, F2>(
        stream: &L,
        state: L::State,
        ch: char,
        is_id_start: F1,
        is_id: F2,
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
        L: Lexer<Token = Self, State = P>,
        F1: Fn(char) -> bool,
        F2: Fn(char) -> bool,
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
    ) -> LexerParseResult<P, Self, L::Error>
    where
        L: CharStream<P>,
        L: Lexer<Token = Self, State = P>,
    {
        for (k, v) in keywords {
            if stream.matches_bytes(&state, k) {
                let n = k.len();
                let next_state = stream.consumed(state, n);
                return Ok(Some((next_state, SimpleToken::Keyword(state, *v))));
            }
        }
        Ok(None)
    }

    //zz All done
}

fn parse_value_fn<
    L: CharStream<TextPos> + Lexer<State = TextPos, Token = LexToken, Error = LexError>,
>(
    stream: &L,
    state: TextPos,
    ch: char,
) -> LexerParseResult<TextPos, LexToken, LexError> {
    let is_digit = |_, ch| ('0'..='9').contains(&ch);
    let (state, opt_x) = stream.do_while(state, ch, &is_digit);
    if let Some((start, _n)) = opt_x {
        let span = StreamCharSpan::new(start, state);
        Ok(Some((state, SimpleToken::Digits(span))))
    } else {
        Ok(None)
    }
}

//a Main
type TextPos = StreamCharPos<LineColumn>;
type LexToken = SimpleToken<TextPos, ()>;
type LexError = SimpleParseError<TextPos>;
type TextStream<'stream> = LexerOfStr<'stream, TextPos, LexToken, LexError>;

type BoxDynLexTokenLexFn<'a> = Box<
    dyn for<'call, 'stream> Fn(
            &'call TextStream<'stream>,
            TextPos,
            char,
        ) -> LexerParseResult<TextPos, LexToken, LexError>
        + 'a,
>;
use std::env;
#[derive(Default)]
struct ParserVec<'a> {
    pub parsers: Vec<BoxDynLexTokenLexFn<'a>>,
}
impl<'a> ParserVec<'a> {
    pub fn new() -> Self {
        let mut parsers = Vec::new();
        Self { parsers }
    }
    pub fn add_parser<F>(&mut self, f: F)
    where
        F: Fn(&TextStream, TextPos, char) -> LexerParseResult<TextPos, LexToken, LexError>
            + 'static,
    {
        self.parsers.push(Box::new(f));
    }
    pub fn add(&mut self) {}
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} <expression>", args[0]));
    }
    let args_as_string = args[1..].join(" ");

    let mut parsers = ParserVec::new();
    parsers.add_parser(|a, b, c| LexToken::parse_whitespace(a, b, c));
    parsers.add_parser(|a, b, c| LexToken::parse_comment_line(a, b, c));
    parsers.add_parser(|a, b, c| LexToken::parse_digits(a, b, c));
    parsers.add_parser(|a, b, c| LexToken::parse_char(a, b, c));

    let l = LexerOfString::default().set_text(args_as_string);
    let ts = l.lexer();
    let tokens = ts.iter(&parsers.parsers);

    println!("Parsing");
    for t in tokens {
        let t = {
            match t {
                Err(e) => {
                    println!();
                    let mut s = String::new();
                    l.fmt_context(&mut s, &e.pos, &e.pos).unwrap();
                    eprintln!("{}", s);
                    return Err(format!("{}", e));
                }
                Ok(t) => t,
            }
        };
        println!("{:?}", t);
    }
    println!();
    println!("Text parsed okay");
    Ok(())
}
