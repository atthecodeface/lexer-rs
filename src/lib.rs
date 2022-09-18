//a Documentation
//a Imports
mod lexer;
mod parser;
pub mod parser_fn;

pub use crate::lexer::{LineColumn, PosnInCharStream, PosnInStream, StreamCharPos, StreamCharSpan};
pub use crate::lexer::{SimpleKeyword, SimpleToken};
pub use crate::lexer::{
    TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult, TokenParser,
    TokenType, TokenTypeError,
};
pub use crate::lexer::{LexerError, LexerState, LexerParseFn, LexerParseResult, Lexer};
pub use crate::lexer::{ParserIterator};

pub use parser::{ParseFnResult, ParseResult};
pub use parser::{ParserInput, ParserInputResult, ParserInputStream};

//a Tests
#[test]
fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;

    type P = usize;
    type K = u64;
    type TSSpan<'a> = TextStreamSpan<'a, P>;
    type Token = SimpleToken<P, K>;
    type E = TokenParseError<P>;

    let span = TSSpan::new(a);
    // Note must use closures here as Rust cannot resolve the lifetimes of the functions otherwise
    let parse_whitespace = |c, b, s| Token::parse_whitespace(c, b, s);
    fn parse_keywords(c: char, b: usize, s: TSSpan) -> TokenParseResult<P, Token, E> {
        let keywords: [(&[u8], u64); 2] = [(b"let", 0), (b"if", 1)];
        Token::parse_keyword(c, b, s, &keywords)
    }
    let parse_id = |c, b, s| Token::parse_id(c, b, s, char::is_alphabetic, char::is_alphanumeric);
    let parsers = [
        |c, b, s| Token::parse_digits(c, b, s),
        parse_whitespace,
        |c, b, s| parse_keywords(c, b, s),
        parse_id,
        |c, b, s| Token::parse_char(c, b, s),
    ];
    for t in span.iter_tokens(&parsers) {
        println!("{:?}", t.unwrap());
    }
    // assert!(false);
}
