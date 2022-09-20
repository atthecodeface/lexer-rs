//a Documentation
/// remove TokenType and use Lexer::Token instead
//a Imports
mod posn_and_span;
mod text_lexer;
mod lexer;
// mod text_stream;
// pub use simple::{SimpleKeyword, SimpleToken};

pub use posn_and_span::LineColumn;
pub use posn_and_span::StreamCharPos;
pub use posn_and_span::StreamCharSpan;
pub use posn_and_span::{PosnInCharStream, PosnInStream};

pub use text_lexer::TSSLexer;

pub use crate::lexer::ParserIterator;
pub use crate::lexer::{Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult, BoxDynLexerParseFn};
pub use crate::lexer::LexerParseError;
