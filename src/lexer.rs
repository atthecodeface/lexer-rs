//a Documentation
//a Imports
mod pos;
mod simple;
mod traits;
mod text_stream;

pub use traits::{PosnInStream, TokenType, TokenTypeError};
pub use pos::{LineCol, Pos, Span};
pub use simple::{SimpleKeyword, SimpleToken};
pub use text_stream::{
    TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult, TokenParser,
};
