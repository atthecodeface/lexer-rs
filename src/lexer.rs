//a Documentation
//a Imports
mod pos;
mod simple;
mod text_stream;

pub use pos::{LineCol, Pos, Span, TextPos};
pub use simple::{SimpleToken, SimpleKeyword};
pub use text_stream::{
    TextStream, TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult,
    TokenParser, TokenType, TokenTypeError,
};

