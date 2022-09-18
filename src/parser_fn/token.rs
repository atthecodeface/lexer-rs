//a Imports
use std::ops::Range;

use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};

//a Map
//fp token_map
/// A parser function generator that allows application of a function
/// to a token, returning Some(R) if the token is matched and maps to
/// a value R, or None if the token is not matched by the parser
///
/// Use cases might be to convert a 'clocked' or 'comb' token to an
/// internal enumeration for a signal type
pub fn token_map<P, I: ParserInputStream<P>, R, F>(f: F) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
    F: Fn(P::Token) -> Option<R>,
{
    use ParseResult::*;
    move |input| {
        match input.get_token()? {
            Some((input, token)) => {
                if let Some(r) = f(token) {
                    return Ok(Matched(input, r));
                }
            }
            _ => (),
        }
        Ok(Mismatched)
    }
}

//a Matches
//fp token_matches
///
pub fn token_matches<P, I: ParserInputStream<P>, F>(f: F) -> impl Fn(I) -> ParseFnResult<P, ()>
where
    P: ParserInput<Stream = I>,
    F: Fn(P::Token) -> bool,
{
    use ParseResult::*;
    move |input| match input.get_token()? {
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

//a Count
//fp token_count
/// A parser function generator that allows application of a match
/// function to a stream of tokens, counting the number of consecutive
/// matches
///
/// If the number of matches 'n' is fewer than the minimum of the range then the parser
/// function does not match; if it is more than that then the match
/// is of Matched with result n, up to the end of the range
///
pub fn token_count<P, I: ParserInputStream<P>, F>(
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
