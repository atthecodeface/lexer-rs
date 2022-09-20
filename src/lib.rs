//a Documentation
/// remove TokenType and use Lexer::Token instead
//a Imports
mod line_column;
mod parser_iter;
// mod simple;
mod stream_char_pos;
mod stream_char_span;
mod text_lexer;
// mod text_stream;
mod token_parse_error;
mod traits;

pub use line_column::LineColumn;
pub use parser_iter::ParserIterator;
// pub use simple::{SimpleKeyword, SimpleToken};
pub use stream_char_pos::StreamCharPos;
pub use stream_char_span::StreamCharSpan;
pub use text_lexer::TSSLexer;
pub use token_parse_error::LexerParseError;
pub use traits::{Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult};
pub use traits::{PosnInCharStream, PosnInStream};
pub use traits::BoxDynLexerPasrseFn;
