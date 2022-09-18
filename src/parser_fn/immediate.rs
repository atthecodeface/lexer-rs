//a Imports
use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};

//a Success, fail and error
//fp success
pub fn success<P, I: ParserInputStream<P>, F, R>(result: F) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn() -> R,
{
    use ParseResult::*;
    move |stream| Ok(Matched(stream, result()))
}

//fp fail
pub fn fail<P, I: ParserInputStream<P>, R>() -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
{
    use ParseResult::*;
    move |_stream| Ok(Mismatched)
}

//fp error
pub fn error<P, I: ParserInputStream<P>, R, E>(e: E) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    E: Fn() -> <P as ParserInput>::Error,
{
    move |_stream| Err(e())
}
