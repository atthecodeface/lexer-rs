//a Imports
use crate::{ParseFnResult, ParseResult, ParserInput, ParserInputStream};

//a First-of
//fp first_of_2/3/4
// Generate a parser function that attempts up to two parsers; if the
// first matches then its result is returned, otherwise the second is
// tried and its result returned. If neither matches then the parser
// function returns a mismatch.
//
// The functions are borrowed, so the returned parser function has a
// lifetime 'b that matches that; the input (lifetime 'a) must
// outlive the resultant parser function
one_f_one_r_slice! { first_of_n, fs, stream {
        for f in fs {
            if let ParseResult::Matched(post_token, token) = f(stream)? {
                return Ok(ParseResult::Matched(post_token, token));
            }
        }
        Ok(ParseResult::Mismatched)
}}

many_f_one_r! { first_of_2, ( f1 : F1, f2 : F2, ) stream {
use ParseResult::*;
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f2(stream)
    }
}
    }

many_f_one_r! { first_of_3, ( f1 : F1, f2 : F2, f3 : F3, ) stream {
use ParseResult::*;
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f2(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f3(stream)
    }
}
    }

many_f_one_r! { first_of_4, ( f1 : F1, f2 : F2, f3 : F3, f4 : F4, ) stream {
use ParseResult::*;
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f2(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f3(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f4(stream)
    }
}
    }
