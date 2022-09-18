//a Documentation
//a Imports
mod pos;
mod line_column;
mod simple;
mod span;
mod traits;
mod text_stream;

pub use line_column::LineColumn;
pub use traits::{PosnInStream, TokenType, TokenTypeError};
pub use pos::Pos;
pub use span::Span;
pub use simple::{SimpleKeyword, SimpleToken};
pub use text_stream::{
    TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult, TokenParser,
};
