//a Imports
use std::ops::Range;

use paste::paste;

use crate::{ParserInput, ParserInputStream, ParseFnResult, ParseResult};

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
//
// separated pair

//a Macros
//mi one_f_one_r - for e.g. first_of_n( [ ] )
// Macro to allow multiple functions with the same return type in a slice
//
// Produces:
//   *  <fn>_dyn_ref([&dyn Fn() -> ParseFnResult<R>]) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else([&dyn Fn() -> ParseFnResult<R>], Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! one_f_one_r_slice {
    ( $fn_name:ident,
      $fs:ident,
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste! {

pub fn [<$fn_name _dyn_ref>] <'b, P, I: ParserInputStream<P>, R, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) ; N]
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [<$fn_name _dyn_ref_else>] <'b, P, I: ParserInputStream<P>, R, G, const N : usize>(
    $fs: [ &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) ; N],
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
{
    move |$stream| {
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
    }
} // pub fn

        } // paste
    }} // macro_rules

//mi many_f_one_r - e.g. for first_of_2/3/4
// Macro to allow multiple functions with the same return type individually
//
// Produces:
//   *  <fn>(f1:F1, f2:F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_else(f1:F1, f2:F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_ref(f1:&F1, f2:&F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_ref_else(f1:&F1, f2:&F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref(f1:&dyn F1, f2:&dyn F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else(f1:&dyn F1, f2:&dyn F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! many_f_one_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident  , )+  $(,)? )
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste! {

pub fn $fn_name<P, I: ParserInputStream<P>, R, $($ft, )*>(
    $( $f : $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R>
    where
        P: ParserInput<Stream = I>,
        $( $ft: Fn(I) -> ParseFnResult<P, R>, )*
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _else >] <P, I: ParserInputStream<P>, R, $($ft, )* G>(
    $( $f : $ft , )*
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R>
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error,
        $( $ft: Fn(I) -> ParseFnResult<P, R>, )*
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

pub fn [< $fn_name _ref>] <'b, P, I: ParserInputStream<P>, R, $($ft, )*>(
    $( $f : &'b $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
            where
                P: ParserInput<Stream = I>,
            $( $ft: Fn(I) -> ParseFnResult<P, R> +'b, )*
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _ref_else>] <'b, P, I: ParserInputStream<P>, R, $($ft, )* G>(
    $( $f : &'b $ft , )*
    g : G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
        $( $ft: Fn(I) -> ParseFnResult<P, R> +'b, )*
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

pub fn [< $fn_name _dyn_ref>] <'b, P, I: ParserInputStream<P>, R>(
    $( $f : &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) , )*
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [< $fn_name _dyn_ref_else>] <'b, P, I: ParserInputStream<P>, R, G>(
    $( $f : &'b (dyn Fn(I) -> ParseFnResult<P, R> +'b) , )*
    g: G,
    ) -> impl Fn(I) -> ParseFnResult<P, R> + 'b
    where
        P: ParserInput<Stream = I>,
        G : Fn() -> <P as ParserInput>::Error + 'b,
{
    move |$stream|
        match ( { $($content)* } )? {
            ParseResult::Mismatched => {
                Err(g())
            }
            x => Ok(x),
        }
} // pub fn

        } // paste
    }} // macro_rules

//mi many_f_many_r - e.g. for pair, tuple3, delimited, etc
// Macro to allow multiple functions with the individual return types
//
// Produces:
//   *  <fn>(f1:F1, f2:F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_else(f1:F1, f2:F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_ref(f1:&F1, f2:&F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_ref_else(f1:&F1, f2:&F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref(f1:&dyn F1, f2:&dyn F2, ...) -> impl ParserFn<P, R>
//   *  <fn>_dyn_ref_else(f1:&dyn F1, f2:&dyn F2, ..., Fn()-> Error) -> impl ParserFn<P, R>
//
macro_rules! many_f_many_r {
    ( $fn_name:ident,
      ( $($f:ident : $ft:ident : $rt:ident),+  $(,)? ),
      $r:ty,
      $stream : ident
      { $($content:tt)* }
    ) => {

        paste! {

pub fn $fn_name<P, I: ParserInputStream<P>, $($rt,)* $($ft, )*>(
    $( $f : $ft , )*
    ) -> impl Fn(I) -> ParseFnResult<P, $r >
    where
        P: ParserInput<Stream = I>,
        $( $ft: Fn(I) -> ParseFnResult<P, $rt>, )*
{
    move |$stream| { $($content)* }
} // pub fn

pub fn [<$fn_name _ref>] <'b, P, I: ParserInputStream<P>, $($rt,)* $($ft, )*>(
    $( $f : &'b $ft , )*
) -> impl Fn(I) -> ParseFnResult<P, $r> + 'b
where
    P: ParserInput<Stream = I>,
    $( $ft: Fn(I) -> ParseFnResult<P, $rt> +'b, )*
{
    move |$stream| { $($content)* }
} // pub fn

        } // paste
    }} // macro_rules

//a Success and fail
//fp success
pub fn success<P, I: ParserInputStream<P>, R: Copy>(result: R) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
{
    use ParseResult::*;
    move |stream| Ok(Matched(stream, result))
}

//fp fail
pub fn fail<P, I: ParserInputStream<P>, R>(_unused: R) -> impl Fn(I) -> ParseFnResult<P, R>
where
    P: ParserInput<Stream = I>,
{
    use ParseResult::*;
    move |stream| Ok(Mismatched)
}

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

//fp match_count
/// A parser function generator that allows application of a match
/// function to a stream of tokens, counting the number of consecutive
/// matches
///
/// If the number of matches 'n' is fewer than 'min' then the parser
/// function does not match; if it is more than 'min' then the match
/// is of Matched with result n
///
pub fn match_count<P, I: ParserInputStream<P>, F>(
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

//a Tuples, preceded, succeeded, delimited
//fp pair
/// Generate a parser function that attempts three parsers in succession
/// which must all match; if they do all match then a Match of the
/// tuple of their results is the response; otherwise a Mismatch
/// occurs.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
many_f_many_r! { pair, ( f1: F1 : R1, f2 : F2 : R2, ), (R1, R2), stream {
    use ParseResult::*;
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

//fp tuple3
many_f_many_r! { tuple3, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), (R1, R2, R3), stream {
    use ParseResult::*;
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

//fp delimited
many_f_many_r! { delimited, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), R2, stream {
    use ParseResult::*;
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

//fp preceded
many_f_many_r! { preceded, ( f1: F1 : R1, f2 : F2 : R2), R2, stream {
    use ParseResult::*;
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

//fp succeeded
many_f_many_r! { succeeded, ( f1: F1 : R1, f2 : F2 : R2), R1, stream {
    use ParseResult::*;
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
