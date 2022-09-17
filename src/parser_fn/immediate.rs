//a Imports
use crate::{ParserInput, ParserInputStream, ParseFnResult, ParseResult};

//a Success, fail and error
//fp success
pub fn success<P, I: ParserInputStream<P>, R: Copy>(result: R) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
{
    use ParseResult::*;
    move |stream| Ok(Matched(stream, result))
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
pub fn error<P, I: ParserInputStream<P>, R>(e: <P as ParserInput>::Error) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    <P as ParserInput>::Error : Copy,
{
    move |_stream| Err(e)
}

