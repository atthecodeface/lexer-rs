//a Documentation
//a Imports
mod pos;
mod simple;
mod text_stream;

pub use pos::{LineCol, Pos, Span, TextPos};
pub use simple::SimpleToken;
pub use text_stream::{
    TextStream, TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult,
    TokenParser, TokenType, TokenTypeError,
};

#[test]
fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;
    let stream = TextStream::new(a);
    let span: TextStreamSpan<u8> = stream.as_span();
    let parsers = [
        SimpleToken::parse_digits,
        SimpleToken::parse_whitespace,
        SimpleToken::parse_char::<TokenParseError<u8>>,
    ];
    for t in span.iter_tokens(&parsers) {
        println!("{:?}", t);
    }
    assert!(false);
}
