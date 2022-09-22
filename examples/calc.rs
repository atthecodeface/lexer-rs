//a Imports
use std::env;

use lexer::LineColumn;
use lexer::SimpleParseError;
use lexer::StreamCharPos;
use lexer::{CharStream, FmtContext, Lexer, LexerOfStr, LexerOfString, LexerParseResult};

//a CalcOp
//tp CalcOp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CalcOp {
    Plus,
    Minus,
    Times,
    Divide,
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

//ip Display for CalcToken
impl std::fmt::Display for CalcToken {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Whitespace => write!(fmt, " "),
            Self::Open => write!(fmt, "("),
            Self::Close => write!(fmt, ")"),
            Self::Op(o) => o.fmt(fmt),
            Self::Value(v) => v.fmt(fmt),
        }
    }
}

//a TextPos, TextStream
//tp TextPos
///
type TextPos = StreamCharPos<LineColumn>;

//tp TextStream
///
type TextStream<'a> = LexerOfStr<'a, TextPos, CalcToken, SimpleParseError<TextPos>>;

//a CalcLexResult
//tp CalcLexResult
// type CalcLexResult = Result<Option<(TPos, Token)>, LexError>;
type CalcLexResult = LexerParseResult<TextPos, CalcToken, SimpleParseError<TextPos>>;

//a Lexical analysis functions
//fi parse_char_fn
/// Parser function to return a Token if it is a known one
/// character token otherwise it returns None
fn parse_char_fn(stream: &TextStream, state: TextPos, ch: char) -> CalcLexResult {
    if let Some(t) = {
        match ch {
            '+' => Some(CalcToken::Op(CalcOp::Plus)),
            '-' => Some(CalcToken::Op(CalcOp::Minus)),
            '*' => Some(CalcToken::Op(CalcOp::Times)),
            '/' => Some(CalcToken::Op(CalcOp::Divide)),
            '(' => Some(CalcToken::Open),
            ')' => Some(CalcToken::Close),
            _ => None,
        }
    } {
        Ok(Some((stream.consumed_char(state, ch), t)))
    } else {
        Ok(None)
    }
}

//fi parse_value_fn
/// Parser function to return a Token if the text matches a value
fn parse_value_fn(stream: &TextStream, state: TextPos, ch: char) -> CalcLexResult {
    let is_digit = |_, ch| ('0'..='9').contains(&ch);
    let (state, opt_x) = stream.do_while(state, ch, &is_digit);
    if let Some((start, _n)) = opt_x {
        let s = stream.get_text(start, state);
        let value: f64 = s.parse().unwrap();
        Ok(Some((state, CalcToken::Value(value))))
    } else {
        Ok(None)
    }
}

//fi parse_whitespace_fn
/// Parser function to return a Token if whitespace
fn parse_whitespace_fn(stream: &TextStream, state: TextPos, ch: char) -> CalcLexResult {
    let is_whitespace = |_n, ch| ch == ' ' || ch == '\t' || ch == '\n';
    let (state, opt_x) = stream.do_while(state, ch, &is_whitespace);
    if let Some((_start, _n)) = opt_x {
        Ok(Some((state, CalcToken::Whitespace)))
    } else {
        Ok(None)
    }
}

//a Main
type BoxDynCalcLexFn<'a> =
    Box<dyn for<'call> Fn(&'call TextStream, TextPos, char) -> CalcLexResult + 'a>;
struct CalcTokenParser<'a> {
    parsers: Vec<BoxDynCalcLexFn<'a>>,
}
impl<'a> CalcTokenParser<'a> {
    pub fn new() -> Self {
        let mut parsers = Vec::new();

        // Note that we use 'as BoxDynCalcLexFn' because type inference kicks in for the Box::new()
        // and does not let parse_value_fn get correctly inferred as dyn Fn(...)
        //
        // Forcing it this way kicks the type inference
        parsers.push(Box::new(parse_value_fn) as BoxDynCalcLexFn);
        parsers.push(Box::new(parse_char_fn) as BoxDynCalcLexFn);
        parsers.push(Box::new(parse_whitespace_fn) as BoxDynCalcLexFn);
        Self { parsers }
    }
    /*
    pub fn add_parser<F: Fn(&TextStream, TextPos, char) -> CalcLexResult + 'a>(&mut self, f: F) {
        self.parsers.push(Box::new(f));
    }*/

    pub fn iter<'iter>(
        &'iter self,
        t: &'iter TextStream<'iter>,
    ) -> impl Iterator<Item = Result<CalcToken, SimpleParseError<TextPos>>> + 'iter {
        t.iter(&self.parsers)
    }
}

//a Main

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} <expression>", args[0]));
    }
    let args_as_string = args[1..].join(" ");
    let c = CalcTokenParser::new();
    let l = LexerOfString::default().set_text(args_as_string);
    let ts = l.lexer();

    // let ts = TextStream::new(&args_as_string);

    println!("Parsing");
    let tokens = c.iter(&ts);
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
        print!("{}", t);
    }
    println!();
    println!("Text parsed okay");
    Ok(())
}

//a Tests - run with cargo test --examples
#[test]
fn test_lex_0() {
    let ts = TextStream::new("1+3");
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
    let mut ts = TextStream::new("2() \t-\n*+/");
    for exp_t in [
        CalcToken::Value(2.0),
        CalcToken::Open,
        CalcToken::Close,
        CalcToken::Whitespace,
        CalcToken::Op(Op::Minus),
        CalcToken::Whitespace,
        CalcToken::Op(Op::Times),
        CalcToken::Op(Op::Plus),
        CalcToken::Op(Op::Divide),
    ] {
        let (next_ts, t) = ts.get_token().unwrap().unwrap();
        assert_eq!(t, exp_t);
        ts = next_ts;
    }
    let x = ts.get_token().unwrap();
    assert!(x.is_none());
}
