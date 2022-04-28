//a Imports
use lexer::{TextStream, TextStreamSpan, TextPos};
use lexer::{TokenParseResult, TokenTypeError, TokenParseError};

use std::collections::HashMap;


//a Pos
//tp Pos
#[derive(Debug, Clone, Copy, Default)]
struct Pos (());

//ip TextPos of Pos
impl TextPos for Pos {}

//ip Display for Pos
impl std::fmt::Display for Pos {
    fn fmt(&self, fmt:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        Ok(())
    }
}

//a Token, Stream, PFnError, PResult
//tp Token
/// Token for this grammar is just a character
type Token = char;

//tp Stream
/// Stream is a text stream using Pos = {} as postition
type Stream<'a> = TextStreamSpan<'a, Pos>;

//tp PFnError
/// The error type returned by the TokenParser that we use
///
/// Probably this can be anything that supports TokenTypeError
type PFnError = TokenParseError<Pos>;

//tp PResult
/// Result of a parser
///
/// This requires a lifetime which is (I believe) what _Bar is about
#[derive(Debug)]
enum PResult<'a, I : PFnInput<'a>, R> {
    Mismatched,
    Matched( I, R ),
    _Bar(std::convert::Infallible, &'a std::marker::PhantomData<usize>),
}

//tt PFnResult
/// Result of a parser function
///
/// This is Err(E) if the parser has a fatal error; Ok(Mismatched) if
/// the parser does not match; Ok(Matched(stream, R)) if a match has
/// occurred - with the stream in the result being now beyond the
/// parsed tokens
///
/// 'a is the lifetime of the input (and R?)
type PFnResult<'a, I, R> = Result<PResult<'a, I, R>, PFnError>;

//tt PFnInput
/// Trait required by a parser of its input
///
/// The parser invokes this to get the tokens that it needs to match
trait PFnInput<'a> : Sized + Copy {
    fn get_token(self) -> PFnInputResult<'a, Self>;
}

//tp PFnInputResult
/// Result of a Parser function given a particular input
///
/// I:PFnInput<'a>
type PFnInputResult<'a, I> = Result<Option<(I, Token)>, PFnError>;

//a AbcTokenStream
//tp AbcTokenStream
/// A stream of tokens of a, b or c
#[derive(Debug, Copy, Clone)]
struct AbcTokenStream<'a> {
    stream : Stream<'a>,
}

//ip AbcTokenStream
impl <'a> AbcTokenStream <'a> {
    //fi parse_char_fn
    /// Parser function to return a Token (== char) if it is one of a-c; otherwise it returns None
    fn parse_char_fn( ch: char,  byte_ofs: usize, stream: Stream ) -> Result<Option<(Stream, Token)>, PFnError> {
        let pos = stream.pos();
        if ('a'..='c').contains(&ch) {
            Ok(Some((stream.consume_char(byte_ofs, ch), ch)))
        } else {
            Ok(None)
        }
    }
}

//ip PFnInput for AbcTokenStream
impl <'a> PFnInput<'a> for AbcTokenStream <'a> {
    //
    fn get_token(self) -> Result<Option<(Self, char)>, PFnError> {
        Ok( self.stream.parse( &[Self::parse_char_fn] )?
            .map(|(stream, t)| (Self {stream}, t)) )
    }
}

//a pfn_*
//fp pfn_map_token
/// A parser function generator that allows application of a function
/// to a token, returning Some(R) if the token is matched and maps to
/// a value R, or None if the token is not matched by the parser
///
/// Use cases might be to convert a 'clocked' or 'comb' token to an
/// internal enumeration for a signal type
fn pfn_map_token<'a, I, R, F>(f:F) -> impl Fn(I) -> PFnResult<'a, I, R>
where I:PFnInput<'a>,
      F:Fn(Token) -> Option<R>,
{
    use PResult::*;
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

//fp pfn_token_match_count
/// A parser function generator that allows application of a match
/// function to a stream of tokens, counting the number of consecutive
/// matches
///
/// If the number of matches 'n' is fewer than 'min' then the parser
/// function does not match; if it is more than 'min' then the match
/// is of Matched with result n
///
/// TODO: change to a Range
fn pfn_token_match_count<'a, I, F>(f:F, min:usize, max:usize) -> impl Fn(I) -> PFnResult<'a, I, usize>
where I:PFnInput<'a>,
      F:Fn(Token) -> bool,
{
    use PResult::*;
    assert!(max >= min);
    move |mut input| {
        let mut n = 0;
        while n < max {
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
        if n >= min {
            Ok(Matched(input, n))
        } else {
            Ok(Mismatched)
        }
    }
}

//fp pfn_first_of_2_ref
/// Generate a parser function that attempts up to two parsers; if the
/// first matches then its result is returned, otherwise the second is
/// tried and its result returned. If neither matches then the parser
/// function returns a mismatch.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
fn pfn_first_of_2_ref<'a, 'b, R, I, F1, F2>( f1: &'b F1, f2: &'b F2 )                                                                                     -> impl Fn(I) -> PFnResult<'a, I, R> + 'b
where
    'a : 'b,
    I : PFnInput<'a>,
    F1 : Fn(I) -> PFnResult<'a, I, R>,
    F2 : Fn(I) -> PFnResult<'a, I, R>,
    {
    use PResult::*;
    move |stream| {
        match f1(stream)? {
            Matched(post_token, token) => {
                return Ok(Matched(post_token, token));
            }
            _ => (),
        }
        f2(stream)
    }
}

//fp pfn_pair_ref
/// Generate a parser function that attempts two parsers in succession
/// which must both match; if they do both match then a Match of the
/// 2-tuple of their results is the response; otherwise a Mismatch
/// occurs.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
fn pfn_pair_ref<'a, 'b, R1, R2, I, F1, F2>( f1:&'b F1, f2:&'b F2 )
                                     -> impl Fn(I) -> PFnResult<'a, I, (R1, R2)> + 'b
where
    'a : 'b,
    I : PFnInput<'a>,
    F1 : Fn(I) -> PFnResult<'a, I, R1>,
    F2 : Fn(I) -> PFnResult<'a, I, R2>,
    {
    use PResult::*;
    move |stream| {
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a,b) => (a,b),
                _ => {return Ok(Mismatched);}
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a,b) => (a,b),
                _ => {return Ok(Mismatched);}
            }
        };
        Ok(Matched(stream, (r1,r2)))
    }
}

//fp pfn_tuple3_ref
/// Generate a parser function that attempts three parsers in succession
/// which must all match; if they do all match then a Match of the
/// tuple of their results is the response; otherwise a Mismatch
/// occurs.
///
/// The functions are borrowed, so the returned parser function has a
/// lifetime 'b that matches that; the input (lifetime 'a) must
/// outlive the resultant parser function
fn pfn_tuple3_ref<'a, 'b, R1, R2, R3, I, F1, F2, F3>( f1:&'b F1, f2:&'b F2, f3:&'b F3 )
                                                 -> impl Fn(I) -> PFnResult<'a, I, (R1, R2, R3)> + 'b
where
    'a: 'b,
    I : PFnInput<'a>,
    F1 : Fn(I) -> PFnResult<'a, I, R1> + 'b,
    F2 : Fn(I) -> PFnResult<'a, I, R2> + 'b,
    F3 : Fn(I) -> PFnResult<'a, I, R3> + 'b,
    {
    use PResult::*;
    move |stream| {
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a,b) => (a,b),
                _ => {return Ok(Mismatched);}
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a,b) => (a,b),
                _ => {return Ok(Mismatched);}
            }
        };
        let (stream, r3) = {
            match f3(stream)? {
                Matched(a,b) => (a,b),
                _ => {return Ok(Mismatched);}
            }
        };
        Ok(Matched(stream, (r1,r2,r3)))
    }
}

//a Tests
//fi test_me
#[test]
fn test_me() {
    let a = r##"aabbbc"##;
    
    let text = TextStream::new(a);
    let stream : Stream  = text.as_span();
    let abcs = AbcTokenStream { stream };

    let is_a = pfn_map_token( |t| if (t == 'a') {Some('a')} else {None} );
    let at_least_one_a = pfn_token_match_count( |t| (t == 'a'), 1, 1000);
    let some_bs = pfn_token_match_count( |t| (t == 'b'), 0, 1000);
    let at_least_one_c = pfn_token_match_count( |t| (t == 'c'), 1, 1000);
    let grammar1 = pfn_tuple3_ref( &at_least_one_a, &some_bs, &at_least_one_c );
    let grammar2 = pfn_tuple3_ref( &at_least_one_c, &some_bs, &at_least_one_a );
    let either_grammar = pfn_first_of_2_ref( &grammar2, &grammar1 );
    println!("{:?}", is_a(abcs));
    println!("{:?}", at_least_one_a(abcs));
    println!("{:?}", grammar1(abcs));
    println!("{:?}", grammar2(abcs));    
    println!("{:?}", either_grammar(abcs));    
    assert!(false);
}

