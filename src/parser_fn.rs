//a Imports
use crate::{Parser, ParserFnInput, ParserFnResult, ParserResult};
use std::ops::Range;

//a map_token
//fp map_token
/// A parser function generator that allows application of a function
/// to a token, returning Some(R) if the token is matched and maps to
/// a value R, or None if the token is not matched by the parser
///
/// Use cases might be to convert a 'clocked' or 'comb' token to an
/// internal enumeration for a signal type
pub fn map_token<'a, P, I: ParserFnInput<'a, P>, R, F>(
    f: F,
) -> impl Fn(I) -> ParserFnResult<'a, P, R>
where
    P: Parser<'a, Input = I>,
    F: Fn(P::Token) -> Option<R>,
{
    use ParserResult::*;
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

//fp match_count
/// A parser function generator that allows application of a match
/// function to a stream of tokens, counting the number of consecutive
/// matches
///
/// If the number of matches 'n' is fewer than 'min' then the parser
/// function does not match; if it is more than 'min' then the match
/// is of Matched with result n
///
pub fn match_count<'a, P, I: ParserFnInput<'a, P>, F>(
    f: F,
    range: Range<usize>,
) -> impl Fn(I) -> ParserFnResult<'a, P, usize>
where
    P: Parser<'a, Input = I>,
    F: Fn(P::Token) -> bool,
{
    use ParserResult::*;
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

//a First-of
//fp first_of_2_ref
/// Generate a parser function that attempts up to two parsers; if the
/// first matches then its result is returned, otherwise the second is
/// tried and its result returned. If neither matches then the parser
/// function returns a mismatch.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
pub fn first_of_2_ref<'a, 'b, P, I: ParserFnInput<'a, P>, R, F1, F2>(
    f1: &'b F1,
    f2: &'b F2,
) -> impl Fn(I) -> ParserFnResult<'a, P, R> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    F1: Fn(I) -> ParserFnResult<'a, P, R> + 'b,
    F2: Fn(I) -> ParserFnResult<'a, P, R> + 'b,
{
    use ParserResult::*;
    move |stream| {
        if let Matched(post_token, token) = f1(stream)? {
            Ok(Matched(post_token, token))
        } else {
            f2(stream)
        }
    }
}

//a Tuple
//fp tuple3_ref
/// Generate a parser function that attempts three parsers in succession
/// which must all match; if they do all match then a Match of the
/// tuple of their results is the response; otherwise a Mismatch
/// occurs.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
pub fn tuple3_ref<'a, 'b, P, I: ParserFnInput<'a, P>, R1, R2, R3, F1, F2, F3>(
    f1: &'b F1,
    f2: &'b F2,
    f3: &'b F3,
) -> impl Fn(I) -> ParserFnResult<'a, P, (R1, R2, R3)> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    F1: Fn(I) -> ParserFnResult<'a, P, R1> + 'b,
    F2: Fn(I) -> ParserFnResult<'a, P, R2> + 'b,
    F3: Fn(I) -> ParserFnResult<'a, P, R3> + 'b,
{
    use ParserResult::*;
    move |stream| {
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, (r1, r2, r3)))
    }
}
