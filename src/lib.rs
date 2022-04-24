//a Documentation
//a Imports
mod pos;
mod text_stream;
mod simple;

pub use pos::{Pos, Span, LineCol, TextPos};
pub use text_stream::{TextStream, TextStreamSpan, TextStreamSpanIterator, TokenTypeError, TokenType, TokenParser, TokenParseResult, TokenParseError};
pub use simple::SimpleToken;

#[test]
fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;
    let stream = TextStream::new(a);
    let span : TextStreamSpan<u8> = stream.as_span();
    let parsers = [ SimpleToken::parse_digits,
                    SimpleToken::parse_char::<TokenParseError<u8>> ];
    for t in span.iter_tokens(&parsers) {
        println!("{:?}",t);
    }
    assert!(false);
}

    
