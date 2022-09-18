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
pub use r#match::{matches, count_of, list_of};

mod token;
pub use token::{token_map, token_matches, token_count};

mod op;
pub use op::{map, fold, option, not, or_else, unwrap_or_else};

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
// ValueOfFirst N
// e.g. value_of_first_2( (parser_a, 0), (parser_b, 1) )
//
// recognize (matched(x) -> matched(span))
// consumed (matched(x) -> matched((span,x)))
//
// vec() -> Matched(r) -> push(r)
//  optionally requires at least one match
//
// separated_list
//
// list (min size, max size)

