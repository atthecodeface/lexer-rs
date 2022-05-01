//a Imports
use std::ops::Range;

use paste::paste;

use crate::{Parser, ParserFnInput, ParserFnResult, ParserResult};

//a Macros
//mi one_f_one_r
/// Macro to allow multiple functions with the same return type
macro_rules! one_f_one_r_slice {
    ( $fn_name_dyn_ref:ident,
      $fs:ident
      { $($content:tt)* }
    ) => {
pub fn $fn_name_dyn_ref<'b, P, I: ParserFnInput<P>, R, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParserFnResult<P, R> +'b) ; N],
) -> impl Fn(I) -> ParserFnResult<P, R> + 'b
where
    P: Parser<Input = I>,
        { $($content)* }
    }
}

//mi many_f_one_r
/// Macro to allow multiple functions with the same return type
macro_rules! many_f_one_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident  , )+  $(,)? )
      { $($content:tt)* }
    ) => {
paste! {
pub fn $fn_name<P, I: ParserFnInput<P>, R, $($ft, )*>(
    $( $f : $ft , )*
) -> impl Fn(I) -> ParserFnResult<P, R>
where
    P: Parser<Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<P, R>, )*
        { $($content)* }

pub fn [< $fn_name _ref>] <'b, P, I: ParserFnInput<P>, R, $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParserFnResult<P, R> + 'b
where
    P: Parser<Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<P, R> +'b, )*
        { $($content)* }

pub fn [< $fn_name _dyn_ref>] <'b, P, I: ParserFnInput<P>, R>(
    $( $f : &'b (dyn Fn(I) -> ParserFnResult<P, R> +'b) , )*
) -> impl Fn(I) -> ParserFnResult<P, R> + 'b
where
    P: Parser<Input = I>,
        { $($content)* }

}
    }
}
//mi many_f_many_r
/// Macro to allow multiple functions with the individual return types
macro_rules! many_f_many_r {
    ( $fn_name:ident, $fn_name_ref:ident,
      ( $($f:ident : $ft:ident : $rt:ident),+  $(,)? ),
      $r:ty,
      { $($content:tt)* }
    ) => {
pub fn $fn_name<P, I: ParserFnInput<P>, $($rt,)* $($ft, )*>(
    $( $f : $ft , )*
) -> impl Fn(I) -> ParserFnResult<P, $r >
where
    P: Parser<Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<P, $rt>, )*
        { $($content)* }

pub fn $fn_name_ref<'b, P, I: ParserFnInput<P>, $($rt,)* $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParserFnResult<P, $r> + 'b
where
    P: Parser<Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<P, $rt> +'b, )*
        { $($content)* }
    }
}

//a map_token
//fp map_token
/// A parser function generator that allows application of a function
/// to a token, returning Some(R) if the token is matched and maps to
/// a value R, or None if the token is not matched by the parser
///
/// Use cases might be to convert a 'clocked' or 'comb' token to an
/// internal enumeration for a signal type
pub fn map_token<P, I: ParserFnInput<P>, R, F>(
    f: F,
) -> impl Fn(I) -> ParserFnResult<P, R>
where
    P: Parser<Input = I>,
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
pub fn match_count<P, I: ParserFnInput<P>, F>(
    f: F,
    range: Range<usize>,
) -> impl Fn(I) -> ParserFnResult<P, usize>
where
    P: Parser<Input = I>,
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
//fp first_of_2/3/4
/// Generate a parser function that attempts up to two parsers; if the
/// first matches then its result is returned, otherwise the second is
/// tried and its result returned. If neither matches then the parser
/// function returns a mismatch.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
one_f_one_r_slice! { first_of_n_dyn_ref, fs {
use ParserResult::*;
    move |stream| {
        for f in fs {
            if let Matched(post_token, token) = f(stream)? {
                return Ok(Matched(post_token, token));
            }
        }
        Ok(Mismatched)
    }
}}

many_f_one_r! { first_of_2, ( f1 : F1, f2 : F2, ) {
use ParserResult::*;
move |stream| {
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f2(stream)
    }
} }
    }

many_f_one_r! { first_of_3, ( f1 : F1, f2 : F2, f3 : F3, ) {
use ParserResult::*;
move |stream| {
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f2(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f3(stream)
    }
} }
    }

many_f_one_r! { first_of_4, ( f1 : F1, f2 : F2, f3 : F3, f4 : F4, )  {
use ParserResult::*;
move |stream| {
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f2(stream)? {
        Ok(Matched(post_token, token))
    } else if let Matched(post_token, token) = f3(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f4(stream)
    }
} }
    }

//a Tuples, preceded, succeeded, delimited
//fp tuple3_ref
/// Generate a parser function that attempts three parsers in succession
/// which must all match; if they do all match then a Match of the
/// tuple of their results is the response; otherwise a Mismatch
/// occurs.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
many_f_many_r! { pair, pair_ref, ( f1: F1 : R1, f2 : F2 : R2, ), (R1, R2), {
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
        Ok(Matched(stream, (r1, r2)))
    }
}
}

many_f_many_r! { tuple3, tuple3_ref, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), (R1, R2, R3), {
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
}

many_f_many_r! { delimited, delimited_ref, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), R2, {
    use ParserResult::*;
    move |stream| {
        let (stream, _r1) = {
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
        let (stream, _r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, r2))
    }
}
}

many_f_many_r! { preceded, preceded_ref, ( f1: F1 : R1, f2 : F2 : R2), R2, {
    use ParserResult::*;
    move |stream| {
        let (stream, _r1) = {
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
        Ok(Matched(stream, r2))
    }
}
}

many_f_many_r! { succeded, succeded_ref, ( f1: F1 : R1, f2 : F2 : R2), R1, {
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
        let (stream, _r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, r1))
    }
}
}
