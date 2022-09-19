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
pub use token_parse_error::{TokenParseError};
pub use text_lexer::{TSSLexer};
pub use traits::{PosnInCharStream, PosnInStream, TokenTypeError};
pub use traits::{LexerError, LexerParseFn, LexerParseResult, Lexer, LexerOfChar};
