//a Imports
use lexer::{LineColumn};
use lexer::{StreamCharPos};
use lexer::{TokenParseError};
use lexer::{TSSLexer, LexerParseResult};

//a CalcOp
//tp CalcOp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CalcOp {
    Plus,
    Minus,
    Times,
    Divide,
}

//ip CalcOp
impl CalcOp {
    fn evaluate(&self, v1: f64, v2: f64) -> Result<f64, String> {
        match self {
            Self::Plus => Ok(v1 + v2),
            Self::Minus => Ok(v1 - v2),
            Self::Times => Ok(v1 * v2),
            Self::Divide => Ok(v1 / v2),
        }
    }
}

//ip Display for CalcOp
impl std::fmt::Display for CalcOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Plus => write!(fmt, "+"),
            Self::Minus => write!(fmt, "-"),
            Self::Times => write!(fmt, "*"),
            Self::Divide => write!(fmt, "/"),
        }
    }
}

//a CalcToken (and sub-enus)
//tp CalcToken
#[derive(Debug, Clone, Copy, PartialEq)]
enum CalcToken {
    Whitespace,
    Open,
    Close,
    Op(CalcOp),
    Value(f64),
}

//a TextPos, TextStream
//tp TextPos
///
type TextPos = StreamCharPos<LineColumn>;

//tp TextStream
///
type TextStream<'a> = TSSLexer<'a, TextPos, CalcToken, TokenParseError<TextPos>>;

//a Lexical analysis functions
//tp CalcLexResult
// type CalcLexResult = Result<Option<(TPos, Token)>, LexError>;
type CalcLexResult<'a> = LexerParseResult<TextStream<'a>>;

//fi parse_char_fn
/// Parser function to return a Token if it is a known one
/// character token otherwise it returns None
fn parse_char_fn<'a>(stream: &TextStream<'a>, state:TextPos, ch:char) -> CalcLexResult<'a>
where
    'a: 'a,
{
    if let Some(t) = {
        match ch {
            '+' => Some(CalcToken::Op(CalcOp::Plus)),
            '-' => Some(CalcToken::Op(CalcOp::Minus)),
            '*' => Some(CalcToken::Op(CalcOp::Times)),
            '/' => Some(CalcToken::Op(CalcOp::Divide)),
            '(' => Some(CalcToken::Open),
            ')' => Some(CalcToken::Close),
            ';' => Some(CalcToken::Semicolon),
            _ => None,
        }
    } {
        Ok(Some((stream.consume_char(state, ch), t)))
    } else {
        Ok(None)
    }
}

//fi parse_value_fn
/// Parser function to return a Token if the text matches a value
fn parse_value_fn<'a>(stream: &TextStream<'a>, state:TextPos, ch:char) -> CalcLexResult<'a>
where
    'a: 'a,
{
    let is_digit = |_, ch| ('0'..='9').contains(&ch);
    let (state, opt_x) = stream.do_while(state, ch, &is_digit);
    if let Some((start, _n)) = opt_x {
        let s = stream.tss.get_text(start, state);
        let value: f64 = s.parse().unwrap();
        Ok(Some((state, CalcToken::Value(value))))
    } else {
        Ok(None)
    }
}

//fi parse_whitespace_fn
/// Parser function to return a Token if whitespace
fn parse_whitespace_fn<'a>(stream: &TextStream<'a>, state:TextPos, ch:char) -> CalcLexResult<'a>
where
    'a: 'a,
{
    let is_whitespace = |_n, ch| ch == ' ' || ch == '\t' || ch == '\n';
    let (state, opt_x) = stream.do_while(state, ch, &is_whitespace);
    if let Some((_start, _n)) = opt_x {
        Ok(Some((state, CalcToken::Whitespace)))
    } else {
        Ok(None)
    }
}


//a Lexical analyzer tests
#[test]
fn test_lex_0() {
    let ts = TokenStream::new("1+3");
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, CalcToken::Value(1.0));
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, CalcToken::Op(Op::Plus));
    let (ts, t) = ts.get_token().unwrap().unwrap();
    assert_eq!(t, CalcToken::Value(3.0));
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}

#[test]
fn test_lex_1() {
    let mut ts = TokenStream::new("2() \t-\n+*/");
    for exp_t in [
        CalcToken::Value(2.0),
        CalcToken::Open,
        CalcToken::Close,
        CalcToken::Whitespace,
        CalcToken::Op(Op::Minus),
        CalcToken::Whitespace,
        CalcToken::Op(Op::Plus),
        CalcToken::Op(Op::Times),
        CalcToken::Op(Op::Divide),
    ] {
        let (next_ts, t) = ts.get_token().unwrap().unwrap();
        assert_eq!(t, exp_t);
        ts = next_ts;
    }
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}

fn main() {
}
