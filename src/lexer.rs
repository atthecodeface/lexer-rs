//a Documentation
//a Imports
mod pos;
mod line_column;
mod simple;
mod span;
mod traits;
mod text_stream;

pub use line_column::LineColumn;
pub use traits::{PosnInStream, PosnInCharStream, TokenType, TokenTypeError};
pub use pos::StreamCharPos;
pub use span::StreamCharSpan;
pub use simple::{SimpleKeyword, SimpleToken};
pub use text_stream::{
    TextStreamSpan, TextStreamSpanIterator, TokenParseError, TokenParseResult, TokenParser,
};
