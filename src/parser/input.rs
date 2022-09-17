//a Imports
use crate::lexer::{TextPos, TokenTypeError};

//a ParserInput, ParserInputResult, ParserInputStream
//tt ParserInput
/// A parser defined using associated types
///
/// It is possible to change the type input to be Sized + Copy, and to
/// pull the 'get_token' function into the Parser trait itself rather
/// than applying it to the Input type.
///
/// However, if that is done then there is nothing tying the Input to
/// the Parser type, only the Parser to the Input; the get_token
/// invocation becomes a P::get_token(input), which is fine, but any
/// use of a parser function (which requires P:Parser and I:Input to
/// be specified) cannot derive the P type from the I type (which is
/// well known as the input to the get_token function). Hence going
/// down that path requires a lot more turbofish on parser_fn
/// invocations to specifiy the Parser itself, which is an
/// anti-pattern.
pub trait ParserInput: Sized {
    /// The type of tokens that are parsed by the Parser; a stream of
    /// these is effectively the input stream to the Parser
    type Token;

    /// An error type that the parser returns if there is a failure to
    /// parse a input stream of tokens
    ///
    /// Using the lexer the Error type will often be:
    ///
    ///    Error : lexer::TokenTypeError<Pos : lexer::TextPos>
    /// 
    /// Pos is the type of a position in an input that needs to be
    /// reported for errors, or that is traced within the Tokens;
    /// often this is the file and start/end lines/characters of the
    /// token
    ///
    /// 
    type Error; // : TokenTypeError<Self::Pos>;

    /// The input type that provides the 'get_token' function to
    /// attempt to provide the next token for a parser to try to
    /// consume.  This type must support Clone as cheaply as possible,
    /// ideally it should be Copy. This is because the parser must
    /// keep copies of the input state so that it can backtrack on
    /// parsing failures.
    type Stream: ParserInputStream<Self>;
}

//tp ParserInputResult
/// This is the result of the Parser::Stream::get_token function,
/// which takes 
///
/// P:Parser
pub type ParserInputResult<P> =
    Result<Option<(<P as ParserInput>::Stream, <P as ParserInput>::Token)>, <P as ParserInput>::Error>;

//tt ParserInputStream
/// Trait required by a parser of its input
///
/// The parser invokes this to get the tokens that it needs to match;
/// making it belong to the Input allows a get_token() call to infer
/// the type of the Parser that it is associated with, reducing
/// turbofish annotation requirements.
///
/// Requiring Copy here allows parser functions to manipulate the
/// input simply without explicit cloning
pub trait ParserInputStream<P: ParserInput<Stream = Self>>: Copy {
    fn get_token(&self) -> ParserInputResult<P>;
}

