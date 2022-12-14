//a Imports
use lexer::parser_fn;
use lexer::{LineColumn, TokenType};
use lexer::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};
use lexer::{PosnInCharStream, StreamCharPos, TextStreamSpan};
use lexer::{TokenParseError, TokenTypeError};
use lexer::{TSSLexer};

//a TextStream
//tp TextStream
///
type TPos = StreamCharPos<LineColumn>;
type TextStream<'a> = TSSLexer<'a, TPos, Token, LexError>;

//a LexError
//tp LexError
#[derive(Debug, Clone, PartialEq, Eq)]
enum LexError {
    Token(TokenParseError<StreamCharPos<LineColumn>>),
    BadChar(char, LineColumn),
    Failure,
    // Other(String),
}

//ip Display for LexError
impl std::fmt::Display for LexError {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

//ip Error for LexError
impl std::error::Error for LexError {}

//ip TokenTypeError for LexError
impl TokenTypeError<StreamCharPos<LineColumn>> for LexError {
    fn failed_to_parse(ch: char, pos: StreamCharPos<LineColumn>) -> Self {
        Self::BadChar(ch, pos.pos())
    }
}
impl <'a> lexer::LexerError<TextStream<'a>> for LexError {
    fn failed_to_parse(_: &TextStream<'a>, _: TPos, _: char) -> Self { todo!() }
}


//a Token (and sub-enus)
//tp Token
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Whitespace,
    Let,
    Semicolon,
    Open,
    Close,
    Op(Op),
    Value(f64),
    Id(usize, usize),
}
impl TokenType for Token {}

//a Lexical analysis functions
//tp LexResult
type LexResult = Result<Option<(TPos, Token)>, LexError>;

//fi parse_char_fn
/// Parser function to return a Token if it is a known one
/// character token otherwise it returns None
fn parse_char_fn<'a>(stream: &TextStream<'a>, state:TPos, ch:char) -> LexResult
where
    'a: 'a,
{
    if let Some(t) = {
        match ch {
            '+' => Some(Token::Op(Op::Plus)),
            '-' => Some(Token::Op(Op::Minus)),
            '*' => Some(Token::Op(Op::Times)),
            '/' => Some(Token::Op(Op::Divide)),
            '(' => Some(Token::Open),
            ')' => Some(Token::Close),
            ';' => Some(Token::Semicolon),
            _ => None,
        }
    } {
        Ok(Some((stream.consume_char(state, ch), t)))
    } else {
        Ok(None)
    }
}

//fi parse_id_fn
/// Parser function to return a Token if the text matches an id
fn parse_id_fn<'a>(stream: &TextStream<'a>, state:TPos, ch:char) -> LexResult
where
    'a: 'a,
{
    let is_start_id = |ch| ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_';
    let is_digit = |ch| ('0'..='9').contains(&ch);
    let is_valid_id = |n, ch| is_start_id(ch) || ((n > 0) && is_digit(ch));
    let (state, opt_x) = stream.do_while(state, ch, &is_valid_id);
    if let Some((start, n)) = opt_x {
        Ok(Some((state, Token::Id(start.byte_ofs(), n))))
    } else {
        Ok(None)
    }
}

//fi parse_value_fn
/// Parser function to return a Token if the text matches a value
fn parse_value_fn<'a>(stream: &TextStream<'a>, state:TPos, ch:char) -> LexResult
where
    'a: 'a,
{
    let is_digit = |_, ch| ('0'..='9').contains(&ch);
    let (state, opt_x) = stream.do_while(state, ch, &is_digit);
    if let Some((start, _n)) = opt_x {
        let s = stream.tss.get_text(start, state);
        let value: f64 = s.parse().unwrap();
        Ok(Some((state, Token::Value(value))))
    } else {
        Ok(None)
    }
}

//fi parse_whitespace_fn
/// Parser function to return a Token if whitespace
fn parse_whitespace_fn<'a>(stream: &TextStream<'a>, state:TPos, ch:char) -> LexResult
where
    'a: 'a,
{
    let is_whitespace = |_n, ch| ch == ' ' || ch == '\t' || ch == '\n';
    let (state, opt_x) = stream.do_while(state, ch, &is_whitespace);
    if let Some((_start, _n)) = opt_x {
        Ok(Some((state, Token::Whitespace)))
    } else {
        Ok(None)
    }
}

//fi parse_keyword_fn
/// Parser function to return a Token if whitespace
fn parse_keyword_fn<'a>(stream: &TextStream<'a>, state:TPos, _ch:char) -> LexResult
where
    'a: 'a,
{
    if stream.matches(&state, "let") {
        state = stream.consume_ascii_str(state, "let");
        Ok(Some((state, Token::Let)))
    } else {
        Ok(None)
    }
}

//a TokenStream
//tp TokenStream
/// A stream of tokens
#[derive(Debug, Clone, Copy)]
struct TokenStream<'a> (TextStream<'a>);

//ip TokenStream
impl<'a> TokenStream<'a> {
    //fp new
    fn new(text: &'a str) -> Self {
        Self(TSSLexer::new(text))
    }

    //mp get_id
    fn get_id(&self, t: &Token) -> &str {
        match t {
            Token::Id(s, n) => {
                // Safety:
                // If the token was 'gotten' correctly then
                // its values indicate utf8 boundaries in the text
                unsafe { self.0.tss.get_text_of_range((*s)..(*s + *n)) }
            }
            _ => {
                panic!("Cannot get id from non-ID token");
            }
        }
    }
}

//tp ParseError
#[derive(Debug)]
struct ParseError();

//ip ParserInputStream for ParserInputTokenStream
#[derive(Debug, Clone, Copy)]
struct ParserInputTokenStream<'a> {
    tss: &'a TextStream<'a>,
    pos : TPos,
}
impl <'a> ParserInputTokenStream<'a> {
    fn new(tss: &'a TextStream<'a>) -> Self {
        let pos = TPos::default();
        Self { tss, pos }
    }
    fn at_pos(&self, pos:TPos) -> Self {
        Self { tss : self.tss,
               pos }
    }
}

//ip ParserInput for TokenStream
impl<'a> ParserInput for TokenStream<'a> {
    type Token = Token;
    type Error = LexError;
    type Stream = ParserInputTokenStream<'a>;
}

//ip ParserInputStream for ParserInputTokenStream
use lexer::Lexer;
impl<'a> ParserInputStream<TokenStream<'a>> for ParserInputTokenStream<'a> {
    //
    fn get_token(&self) -> Result<Option<(Self, Token)>, LexError> {
        let parsers = &[
            parse_whitespace_fn,
            parse_keyword_fn,
            parse_id_fn,
            parse_value_fn,
            parse_char_fn,
        ];
        Ok(self.tss
           .parse( self.pos, parsers)?
           .map(|(state, t)| (self.at_pos(state), t)))
    }
}

//a Lexical analyzer tests
#[test]
fn test_lex_0() {
    let ts = TokenStream::new("1+3");
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, Token::Value(1.0));
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, Token::Op(Op::Plus));
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, Token::Value(3.0));
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}

#[test]
fn test_lex_1() {
    let mut ts = TokenStream::new("2() \t-\n+*/let;");
    for exp_t in [
        Token::Value(2.0),
        Token::Open,
        Token::Close,
        Token::Whitespace,
        Token::Op(Op::Minus),
        Token::Whitespace,
        Token::Op(Op::Plus),
        Token::Op(Op::Times),
        Token::Op(Op::Divide),
        Token::Let,
        Token::Semicolon,
    ] {
        let (next_ts, t) = ts.get_token().unwrap().unwrap();
        assert_eq!(t, exp_t);
        ts = next_ts;
    }
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}

#[test]
fn test_lex_ids() {
    let mut ts = TokenStream::new("x y _h");
    for id in ["x", "y", "_h"] {
        let t = {
            loop {
                let (next_ts, t) = ts.get_token().unwrap().unwrap();
                ts = next_ts;
                if t != Token::Whitespace {
                    break t;
                }
            }
        };
        if let Token::Id(_s, _n) = &t {
            assert_eq!(ts.get_id(&t), id);
        } else {
            panic!(
                "Token should have been an ID of '{}' but it was {:?}",
                id, t
            );
        }
        dbg!(&ts);
    }
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}

//a Scope
struct Scope();
//a Op
//tp Op
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Plus,
    Minus,
    Times,
    Divide,
}

//ip Op
impl Op {
    fn evaluate(&self, v1: f64, v2: f64) -> Result<f64, String> {
        match self {
            Self::Plus => Ok(v1 + v2),
            Self::Minus => Ok(v1 - v2),
            Self::Times => Ok(v1 * v2),
            Self::Divide => Ok(v1 / v2),
        }
    }
}

//ip Display for Op
impl std::fmt::Display for Op {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Plus => write!(fmt, "+"),
            Self::Minus => write!(fmt, "-"),
            Self::Times => write!(fmt, "*"),
            Self::Divide => write!(fmt, "/"),
        }
    }
}

//a Expr
//tp Expr
#[derive(Debug)]
enum Expr {
    Value(f64),
    BinaryOp(Op, Box<Expr>, Box<Expr>),
}

//ip Expr
impl Expr {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Value(v) => Some(Expr::Value(*v)),
            _ => None,
        }
    }
    fn evaluate(&self, scope: &Scope) -> Result<f64, String> {
        match self {
            Self::Value(v) => Ok(*v),
            Self::BinaryOp(o, e1, e2) => {
                let v1 = e1.evaluate(scope)?;
                let v2 = e2.evaluate(scope)?;
                o.evaluate(v1, v2)
            }
        }
    }
}

//ip Display for Expr
impl std::fmt::Display for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Value(v) => write!(fmt, "{}", v),
            Expr::BinaryOp(o, e1, e2) => write!(fmt, "({}) {} ({})", e1, o, e2),
        }
    }
}

//a Parse functions
//fi parse_eof
fn parse_eof(input: ParserInputTokenStream) -> ParseFnResult<ParserInputTokenStream, ()> {
    if input.get_token()?.is_some() {
        Ok(ParseResult::Mismatched)
    } else {
        Ok(ParseResult::Matched(input, ()))
    }
}

//fi parse_leaf
fn parse_leaf(input: ParserInputTokenStream) -> ParseFnResult<ParserInputTokenStream, Expr> {
    let parse_value = parser_fn::token_map(|t| Expr::from_token(&t));
    let parse_bracketed = parser_fn::delimited(
        parser_fn::matches(|t| matches!(t, Token::Open)),
        &parse_expr,
        parser_fn::matches(|t| matches!(t, Token::Close)),
    );
    parser_fn::first_of_2(parse_value, parse_bracketed)(input)
}

//fi parse_binop_1
fn parse_binop_1(input: ParserInputTokenStream) -> ParseFnResult<ParserInputTokenStream, Expr> {
    parser_fn::fold(
        0,
        |_n, v1, (o, v2)| Expr::BinaryOp(o, Box::new(v1), Box::new(v2)),
        &parse_leaf,
        parser_fn::pair(
            &parser_fn::token_map(|t| match t {
                Token::Op(Op::Times) => Some(Op::Times),
                Token::Op(Op::Divide) => Some(Op::Divide),
                _ => None,
            }),
            &parse_leaf,
        ),
    )(input)
}

//fi parse_binop_2
fn parse_binop_2(input: ParserInputTokenStream) -> ParseFnResult<ParserInputTokenStream, Expr> {
    parser_fn::fold(
        0,
        |_n, v1, (o, v2)| Expr::BinaryOp(o, Box::new(v1), Box::new(v2)),
        &parse_binop_1,
        parser_fn::pair(
            &parser_fn::token_map(|t| match t {
                Token::Op(Op::Plus) => Some(Op::Plus),
                Token::Op(Op::Minus) => Some(Op::Minus),
                _ => None,
            }),
            &parse_binop_1,
        ),
    )(input)
}

//fi parse_expr
fn parse_expr(input: ParserInputTokenStream) -> ParseFnResult<ParserInputTokenStream, Expr> {
    parse_binop_2(input)
}

//a Parser tests
#[test]
fn parse_and_evaluate() {
    let parse_expr_eof =
        parser_fn::unwrap_or_else(parser_fn::succeeded_ref(&parse_expr, &parse_eof), || {
            LexError::Failure
        });

    let tss = TextStream::new("1+2*3+4+(5+6)");
    let pits = ParserInputTokenStream::new(&tss);
    let e = parse_expr_eof(pits).expect("Expression should parse cleanly");
    assert_eq!(e.evaluate(&Scope()).unwrap(), 22.0);

    let ts = TokenStream::new("1+2*(3+4)+5+6");
    let pits = ParserInputTokenStream::new(&tss);
    let e = parse_expr_eof(ts).expect("Expression should parse cleanly");
    assert_eq!(e.evaluate(&Scope()).unwrap(), 26.0);

    let ts = TokenStream::new("1+2*(4-3+2)+5+6");
    let pits = ParserInputTokenStream::new(&tss);
    let e = parse_expr_eof(pits).expect("Expression should parse cleanly");
    assert_eq!(e.evaluate(&Scope()).unwrap(), 18.0);

    let ts = TokenStream::new("1++2*(4-3+2)+5+6");
    let pits = ParserInputTokenStream::new(&tss);
    assert_eq!(parse_expr_eof(pits).unwrap_err(), LexError::Failure);
}
