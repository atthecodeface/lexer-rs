//a Imports
use crate::lexer::{TextPos, TokenTypeError};

//a Token, Stream, PFnError, PResult
//tt Parser
/// A parser defined using assocaited types
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
pub trait Parser: Sized {
    type Token;
    type Pos: TextPos;
    type Error: TokenTypeError<Self::Pos>;
    type Input: ParserFnInput<Self>;
}

//tp ParserInputResult
///
/// P:Parser
pub type ParserInputResult<P> =
    Result<Option<(<P as Parser>::Input, <P as Parser>::Token)>, <P as Parser>::Error>;

//tt ParserFnInput
/// Trait required by a parser of its input
///
/// The parser invokes this to get the tokens that it needs to match;
/// making it belong to the Input allows a get_token() call to infer
/// the type of the Parser that it is associated with, reducing
/// turbofish annotation requirements.
///
/// Requiring Copy here allows parser functions to manipulate the
/// input simply without explicit cloning
pub trait ParserFnInput<P: Parser<Input = Self>>: Copy {
    fn get_token(&self) -> ParserInputResult<P>;
}

//tp ParserResult
#[derive(Debug)]
pub enum ParserResult<P: Parser, R> {
    Mismatched,
    Matched(P::Input, R),
}

//tp ParserFnResult
// P:Parser<'a, Error = E>
pub type ParserFnResult<P, R> = Result<ParserResult<P, R>, <P as Parser>::Error>;
