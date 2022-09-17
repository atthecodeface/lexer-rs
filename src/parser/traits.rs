//a Imports
use super::ParserInput;

//a ParseResult, ParseFnResult
//tp ParseResult
#[derive(Debug)]
pub enum ParseResult<P: ParserInput, R> {
    Mismatched,
    Matched(P::Stream, R),
}

//tp ParserFnResult
// P:ParserInput<'a, Error = E>
pub type ParseFnResult<P, R> = Result<ParseResult<P, R>, <P as ParserInput>::Error>;
