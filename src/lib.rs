//a Documentation
mod lexer;
/// remove TokenType and use Lexer::Token instead
//a Imports
mod posn_and_span;
// mod text_stream;
// pub use simple::{SimpleKeyword, SimpleToken};

pub use posn_and_span::LineColumn;
pub use posn_and_span::StreamCharPos;
pub use posn_and_span::StreamCharSpan;
pub use posn_and_span::{PosnInCharStream, PosnInStream};

pub use crate::lexer::LexerOfStr as TSSLexer;

pub use crate::lexer::LexerParseError;
pub use crate::lexer::ParserIterator;
pub use crate::lexer::{
    BoxDynLexerParseFn, Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult,
};
