//a Imports
use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};

#[macro_use]
mod macros;

mod comb;
pub use comb::{delimited, delimited_ref};
pub use comb::{pair, pair_ref};
pub use comb::{preceded, preceded_ref};
pub use comb::{separated_pair, separated_pair_ref};
pub use comb::{succeeded, succeeded_ref};
pub use comb::{tuple3, tuple3_ref};
pub use comb::{tuple4, tuple4_ref};

mod immediate;
pub use immediate::{error, fail, success};

mod r#match;
pub use r#match::{count_of, list_of};

mod priority;
pub use priority::{
    first_of_2, first_of_2_dyn_ref, first_of_2_dyn_ref_else, first_of_2_else, first_of_2_ref,
    first_of_2_ref_else,
};
pub use priority::{
    first_of_3, first_of_3_dyn_ref, first_of_3_dyn_ref_else, first_of_3_else, first_of_3_ref,
    first_of_3_ref_else,
};
pub use priority::{
    first_of_4, first_of_4_dyn_ref, first_of_4_dyn_ref_else, first_of_4_else, first_of_4_ref,
    first_of_4_ref_else,
};
pub use priority::{first_of_n_dyn_ref, first_of_n_dyn_ref_else};

//a TO DO
// not(parser) : mismatch -> Match(), Match -> Mismatched
// ValueOfFirst N
// e.g. value_of_first_2( (parser_a, 0), (parser_b, 1) )
// Map
// e.g. map( parser, fn |x| Thing::Fred(x) )
//
// e.g. map( parser, fn |x| Thing::Fred(x) )
//
// opt (matched(x) -> matched(Some(x)), mismatch -> Matched(none))
// recognize (matched(x) -> matched(span))
// consumed (matched(x) -> matched((span,x)))
//
// fold(init, f) -> Mismatched -> matched(acc); Matched -> fold(f(acc, r))
//  optionally requires at least one match
//
// vec() -> Matched(r) -> push(r)
//  optionally requires at least one match
//
// separated_list
//
// list (min size, max size)

//a map_token
//fp map_token
/// A parser function generator that allows application of a function
/// to a token, returning Some(R) if the token is matched and maps to
/// a value R, or None if the token is not matched by the parser
///
/// Use cases might be to convert a 'clocked' or 'comb' token to an
/// internal enumeration for a signal type
pub fn map_token<P, I: ParserInputStream<P>, R, F>(f: F) -> impl Fn(I) -> ParseFnResult<P, R>
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
