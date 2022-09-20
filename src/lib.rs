//a Documentation
/// remove TokenType and use Lexer::Token instead
//a Imports
mod posn_and_span;
mod parser_iter;
mod text_lexer;
mod token_parse_error;
mod traits;
// mod simple;
// mod text_stream;
// pub use simple::{SimpleKeyword, SimpleToken};

pub use posn_and_span::LineColumn;
pub use posn_and_span::StreamCharPos;
pub use posn_and_span::StreamCharSpan;
pub use posn_and_span::{PosnInCharStream, PosnInStream};

pub use parser_iter::ParserIterator;
pub use text_lexer::TSSLexer;
pub use token_parse_error::LexerParseError;
pub use traits::{Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult};
pub use traits::BoxDynLexerPasrseFn;
