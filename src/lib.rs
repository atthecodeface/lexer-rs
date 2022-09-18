//a Documentation
//a Imports
mod lexer;
mod parser;
pub mod parser_fn;

pub use crate::lexer::{LineColumn, PosnInCharStream, PosnInStream, StreamCharPos, StreamCharSpan};
pub use crate::lexer::{SimpleKeyword, SimpleToken};
pub use crate::lexer::{
    TextStreamSpan, TokenParseError, TokenParseResult, TokenParser,
    TokenType, TokenTypeError, TSSLexer
};
pub use crate::lexer::{LexerError, LexerParseFn, LexerParseResult, Lexer, LexerOfChar};
pub use crate::lexer::{ParserIterator};

pub use parser::{ParseFnResult, ParseResult};
pub use parser::{ParserInput, ParserInputResult, ParserInputStream};

