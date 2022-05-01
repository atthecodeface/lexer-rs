//a Imports
use std::ops::Range;

use paste::paste;

use crate::{Parser, ParserFnInput, ParserFnResult, ParserResult};

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
// success(r) -> Matched(r)
// fail -> Mismatched
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
//
// separated pair

//a Macros
//mi one_f_one_r
/// Macro to allow multiple functions with the same return type
macro_rules! one_f_one_r_slice {
    ( $fn_name:ident,
      $fs:ident,
      $stream : ident
      { $($content:tt)* }
    ) => {
paste! {

    pub fn [<$fn_name _dyn_ref>] <'a, 'b, P, I: ParserFnInput<'a, P>, R, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParserFnResult<'a, P, R> +'b) ; N]
) -> impl Fn(I) -> ParserFnResult<'a, P, R> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    {
    move |$stream| { $($content)* }
} // pub fn

    pub fn [<$fn_name _dyn_ref_else>] <'a, 'b, P, I: ParserFnInput<'a, P>, R, G, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParserFnResult<'a, P, R> +'b) ; N],
    g : G,
) -> impl Fn(I) -> ParserFnResult<'a, P, R> + 'b
where
        P: Parser<'a, Input = I>,
    G : Fn() -> <P as Parser<'a>>::Error + 'b,
    'a: 'b,
    {
    move |$stream| {
        match ( { $($content)* } )? {
            ParserResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
    }
} // pub fn
} // paste
    }} // macro_rules

//mi many_f_one_r
/// Macro to allow multiple functions with the same return type
macro_rules! many_f_one_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident  , )+  $(,)? )
      $stream : ident
      { $($content:tt)* }
    ) => {
paste! {

pub fn $fn_name<'a, P, I: ParserFnInput<'a, P>, R, $($ft, )*>(
    $( $f : $ft , )*
) -> impl Fn(I) -> ParserFnResult<'a, P, R>
where
    P: Parser<'a, Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<'a, P, R>, )*
    {
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _else >] <'a, P, I: ParserFnInput<'a, P>, R, $($ft, )* G>(
    $( $f : $ft , )*
    g : G,
) -> impl Fn(I) -> ParserFnResult<'a, P, R>
where
    P: Parser<'a, Input = I>,
    G : Fn() -> <P as Parser<'a>>::Error,
    $( $ft: Fn(I) -> ParserFnResult<'a, P, R>, )*
    {
    move |$stream|
        match ( { $($content)* } )? {
            ParserResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

pub fn [< $fn_name _ref>] <'a, 'b, P, I: ParserFnInput<'a, P>, R, $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParserFnResult<'a, P, R> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    $( $ft: Fn(I) -> ParserFnResult<'a, P, R> +'b, )*
    {
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _dyn_ref>] <'a, 'b, P, I: ParserFnInput<'a, P>, R>(
    $( $f : &'b (dyn Fn(I) -> ParserFnResult<'a, P, R> +'b) , )*
) -> impl Fn(I) -> ParserFnResult<'a, P, R> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    {
    move |$stream| { $($content)* }
} // pub fn
    }
    }}

//mi many_f_many_r
/// Macro to allow multiple functions with the individual return types
macro_rules! many_f_many_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident : $rt:ident),+  $(,)? ),
      $r:ty,
      { $($content:tt)* }
    ) => {
paste! {
pub fn $fn_name<'a, P, I: ParserFnInput<'a, P>, $($rt,)* $($ft, )*>(
    $( $f : $ft , )*
) -> impl Fn(I) -> ParserFnResult<'a, P, $r >
where
    P: Parser<'a, Input = I>,
    $( $ft: Fn(I) -> ParserFnResult<'a, P, $rt>, )*
        { $($content)* }

pub fn [<$fn_name _ref>]<'a, 'b, P, I: ParserFnInput<'a, P>, $($rt,)* $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParserFnResult<'a, P, $r> + 'b
where
    P: Parser<'a, Input = I>,
    'a: 'b,
    $( $ft: Fn(I) -> ParserFnResult<'a, P, $rt> +'b, )*
        { $($content)* }
    }
    }
}

//a Success and fail
pub fn success<'a, P, I: ParserFnInput<'a, P>, R:Copy>(result:R
) -> impl Fn(I) -> ParserFnResult<'a, P, R>
where
    P: Parser<'a, Input = I>,
{
    use ParserResult::*;
    move |stream| {
        Ok(Matched(stream, result))
    }
}
pub fn fail<'a, P, I: ParserFnInput<'a, P>, R>(_unused:R) -> impl Fn(I) -> ParserFnResult<'a, P, R>
where
    P: Parser<'a, Input = I>,
{
    use ParserResult::*;
    move |stream| {
        Ok(Mismatched)
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
//fp first_of_2/3/4
/// Generate a parser function that attempts up to two parsers; if the
/// first matches then its result is returned, otherwise the second is
/// tried and its result returned. If neither matches then the parser
/// function returns a mismatch.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
one_f_one_r_slice! { first_of_n, fs, stream {
        for f in fs {
            if let ParserResult::Matched(post_token, token) = f(stream)? {
                return Ok(ParserResult::Matched(post_token, token));
            }
        }
        Ok(ParserResult::Mismatched)
}}

many_f_one_r! { first_of_2, ( f1 : F1, f2 : F2, ) stream {
use ParserResult::*;
    if let Matched(post_token, token) = f1(stream)? {
        Ok(Matched(post_token, token))
    } else {
        f2(stream)
    }
}
    }

many_f_one_r! { first_of_3, ( f1 : F1, f2 : F2, f3 : F3, ) stream {
use ParserResult::*;
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
use ParserResult::*;
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
many_f_many_r! { pair, ( f1: F1 : R1, f2 : F2 : R2, ), (R1, R2), {
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

many_f_many_r! { tuple3, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), (R1, R2, R3), {
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

many_f_many_r! { delimited, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), R2, {
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

many_f_many_r! { preceded, ( f1: F1 : R1, f2 : F2 : R2), R2, {
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

many_f_many_r! { succeded, ( f1: F1 : R1, f2 : F2 : R2), R1, {
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
