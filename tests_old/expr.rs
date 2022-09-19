/*
use lexer::{SimpleToken, SimpleKeyword};
use lexer::{TextStream, TextStreamSpan, TextPos};
use lexer::{TokenParseResult, TokenParseError};

use std::collections::HashMap;


#[derive(Debug, Clone, Copy, Default)]
struct Pos (());
impl TextPos for Pos {}
impl std::fmt::Display for Pos {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        Ok(())
    }
}

type Stream<'a> = TextStreamSpan<'a, Pos>;
type Token = SimpleToken<Pos, Keyword>;
type ParseResult<'a> = TokenParseResult<'a, Pos, Token, TokenParseError<Pos>> ;

//a Keywords
#[derive(Debug, Copy, Clone)]
pub enum Keyword {
    Let,
    If,
    Else,
}

impl SimpleKeyword for Keyword {}

#[derive(Debug, Default)]
pub struct Parsed {
    last_id : usize,
    ids: HashMap<String, usize>,
}
type Op = usize;
enum Expression {
    Let(usize, Vec<usize>, Box<Expression>),
    Id(usize),
    Number(isize),
    UnaryOp(Op, Box<Expression>),
    BinaryOp(Op, Box<Expression>),
}

//a Parse functions
fn parse_keywords_fn( c:char, b:usize, s:Stream) -> ParseResult {
    let keywords : &[ (&[u8], Keyword)] = &[
        (b"let", Keyword::Let),
        (b"if", Keyword::If),
        (b"else", Keyword::Else),
    ];
    Token::parse_keyword(c,b,s, keywords)
}


fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;

let parse_keywords = |c,b,s| parse_keywords_fn(c,b,s);
let parse_comment = |c,b,s| Token::parse_comment_line(c,b,s);
let parse_whitespace = |c,b,s| Token::parse_whitespace(c,b,s);
let parse_id = |c,b,s| Token::parse_id(c,b,s, char::is_alphabetic, char::is_alphanumeric);
let parse_number = |c,b,s| Token::parse_digits(c,b,s);
let parse_char = |c,b,s| Token::parse_char(c,b,s);

let parsers = [ parse_keywords, parse_whitespace, parse_comment, parse_number, parse_id, parse_char];

    let stream = TextStream::new(a);
    let span : Stream  = stream.as_span();
    for t in span.iter_tokens(&parsers) {
        println!("{:?}", t.unwrap());
    }
    assert!(false);
}
 */
