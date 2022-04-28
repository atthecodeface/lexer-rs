//a Imports
use crate::lexer::{TextPos, TokenTypeError};

//a Token, Stream, PFnError, PResult
//tt Parser
pub trait Parser<'a>: Sized {
    type Token: 'a;
    type Pos: TextPos;
    type Error: TokenTypeError<Self::Pos> + 'a;
    type Input: ParserFnInput<'a, Self> + 'a;
}

//tp ParserInputResult
///
/// P:Parser<'a>
pub type ParserInputResult<'a, P> =
    Result<Option<(<P as Parser<'a>>::Input, <P as Parser<'a>>::Token)>, <P as Parser<'a>>::Error>;

//tt ParserFnInput
/// Trait required by a parser of its input
///
/// The parser invokes this to get the tokens that it needs to match
pub trait ParserFnInput<'a, P: Parser<'a, Input = Self>>: Sized + Copy {
    fn get_token(self) -> ParserInputResult<'a, P>;
}

//tp ParserResult
#[derive(Debug)]
pub enum ParserResult<'a, P: Parser<'a>, R> {
    Mismatched,
    Matched(P::Input, R),
}

//tp ParserFnResult
// P:Parser<'a, Error = E>
pub type ParserFnResult<'a, P, R> = Result<ParserResult<'a, P, R>, <P as Parser<'a>>::Error>;
