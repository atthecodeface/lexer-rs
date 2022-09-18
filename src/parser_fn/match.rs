//a Imports
use std::ops::Range;

use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};

//a count
//fp matches
///
pub fn matches<P, I: ParserInputStream<P>, F>(f: F) -> impl Fn(I) -> ParseFnResult<P, ()>
where
    P: ParserInput<Stream = I>,
    F: Fn(P::Token) -> bool,
{
    use ParseResult::*;
    move |mut input| match input.get_token()? {
        Some((next_input, token)) => {
            if f(token) {
                Ok(Matched(next_input, ()))
            } else {
                Ok(Mismatched)
            }
        }
        _ => Ok(Mismatched),
    }
}

//fp count_of
/// A parser function generator that allows application of a match
/// function to a stream of tokens, counting the number of consecutive
/// matches
///
/// If the number of matches 'n' is fewer than the minimum of the range then the parser
/// function does not match; if it is more than that then the match
/// is of Matched with result n, up to the end of the range
///
pub fn count_of<P, I: ParserInputStream<P>, F>(
    f: F,
    range: Range<usize>,
) -> impl Fn(I) -> ParseFnResult<P, usize>
where
    P: ParserInput<Stream = I>,
    F: Fn(P::Token) -> bool,
{
    use ParseResult::*;
    move |mut input| {
        let mut n = 0;
        while n < range.end {
            match input.get_token()? {
                Some((next_input, token)) => {
                    if !f(token) {
                        break;
                    }
                    input = next_input;
                    n += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if n >= range.start {
            Ok(Matched(input, n))
        } else {
            Ok(Mismatched)
        }
    }
}

//a List
//fp list_of
/// A parser function generator that allows application of a match
/// function to a stream of tokens, generating a Vec of the results
///
/// If the number of matches 'n' is fewer than the minimum of the range then the parser
/// function does not match; if it is more than that then the match
/// is of Matched with result n, up to the end of the range
///
pub fn list_of<P, I: ParserInputStream<P>, F, R>(
    f: F,
    range: Range<usize>,
) -> impl Fn(I) -> ParseFnResult<P, Vec<R>>
where
    P: ParserInput<Stream = I>,
    F: Fn(I) -> ParseFnResult<P, R>,
{
    use ParseResult::*;
    move |mut input| {
        let mut result = Vec::new();
        let mut n = 0;
        while n < range.end {
            match f(input)? {
                Matched(next_input, r) => {
                    result.push(r);
                    n += 1;
                    input = next_input;
                }
                _ => {
                    break;
                }
            }
        }
        if n >= range.start {
            Ok(Matched(input, result))
        } else {
            Ok(Mismatched)
        }
    }
}
