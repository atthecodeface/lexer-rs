//a Imports
use std::ops::Range;

use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};


//a map_token
//fp map
pub fn map<P, I: ParserInputStream<P>, T, R, F, M>(f: F, m:M) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, T>,
    M : Fn(T) -> R,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(input, t) => 
                Ok(Matched(input, m(t))),
            Mismatched =>
                Ok(Mismatched),
        }
    }
}

//a Fold
//fp fold
pub fn fold<P, I: ParserInputStream<P>, R, F, G, H, T>(max:usize, fold:H, f: F, g:G) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, R>,
    G: Fn(I) -> ParseFnResult<P, T>,
    H: Fn(usize, R, T) -> R,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(mut input, mut t) => {
                let mut n = 0;
                while (max==0) || (n < max) {
                    let (next_input, t2) = {
                        match g(input)? {
                            Matched(next_input, t2) => (next_input, t2),
                            Mismatched => {
                                return Ok(Matched(input, t));
                            }
                        }
                    };
                    t = fold(n, t, t2);
                    n += 1;
                    input = next_input;
                }
                Ok(Matched(input, t))
            }
            Mismatched =>
                Ok(Mismatched),
        }
    }
}

//fp option
pub fn option<P, I: ParserInputStream<P>, R, F>(f: F) -> impl Fn(I) -> ParseFnResult<P, Option<R>>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, R>,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(input, r) => 
                Ok(Matched(input, Some(r))),
            Mismatched =>
                Ok(Matched(input, None)),
        }
    }
}

//fp not
pub fn not<P, I: ParserInputStream<P>, R, T, F>(f: F, r:R) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, T>,
    R: Copy,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(_, _) => 
                Ok(Mismatched),
            Mismatched =>
                Ok(Matched(input, r)),
        }
    }
}

//fp or_else
pub fn or_else<P, I: ParserInputStream<P>, R, F, E>(f: F, e:E) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, R>,
    E: Fn(I) -> ParseFnResult<P, R>,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(input, r) => 
                Ok(Matched(input, r)),
            Mismatched =>
                e(input)
        }
    }
}

//fp unnwrap_or_else
pub fn unwrap_or_else<P, I: ParserInputStream<P>, R, F, E, G>(f: F, e:G) -> impl Fn(I) -> Result<R, E>
where
    P: ParserInput<Stream = I, Error = E>,
    F: Fn(I) -> ParseFnResult<P, R>,
    G: Fn() -> E,
{
    use ParseResult::*;
    move |input| {
        match f(input)? {
            Matched(_input, r) => 
                Ok(r),
            Mismatched =>
                Err(e())
        }
    }
}

