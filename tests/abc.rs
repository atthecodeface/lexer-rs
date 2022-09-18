//a Imports
use lexer::parser_fn;
use lexer::{ParseFnResult, ParserInput, ParserInputStream};
use lexer::{PosnInCharStream, TextStreamSpan};
use lexer::{TokenParseError, TokenTypeError};

//a Pos
//tp Pos
type Pos = usize;

//a AbcTokenStream
//tp AbcTokenStreamError
#[derive(Debug)]
enum AbcTokenStreamError {
    #[allow(dead_code)]
    Token(TokenParseError<Pos>),
    Other(String),
}
impl std::fmt::Display for AbcTokenStreamError {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
impl std::error::Error for AbcTokenStreamError {}
impl TokenTypeError<Pos> for AbcTokenStreamError {
    fn failed_to_parse(_: char, _: Pos) -> Self {
        todo!()
    }
}
impl From<TokenParseError<Pos>> for AbcTokenStreamError {
    fn from(_: TokenParseError<Pos>) -> Self {
        todo!()
    }
}

//tp AbcTokenStream
/// A stream of tokens of a, b or c
#[derive(Debug, Copy, Clone)]
struct AbcTokenStream<'a> {
    stream: TextStreamSpan<'a, Pos>,
}

//ip ParserInput for AbcTokenStream
impl<'a> ParserInput for AbcTokenStream<'a> {
    type Token = char;
    //    type Pos = Pos;
    type Error = AbcTokenStreamError;
    type Stream = AbcTokenStream<'a>;
}

//ip AbcTokenStream
impl<'a> AbcTokenStream<'a> {
    //fi parse_char_fn
    /// Parser function to return a Token (== char) if it is one of a-c; otherwise it returns None
    fn parse_char_fn(
        ch: char,
        byte_ofs: usize,
        stream: TextStreamSpan<Pos>,
    ) -> Result<Option<(TextStreamSpan<Pos>, char)>, TokenParseError<Pos>> {
        if ('a'..='c').contains(&ch) {
            Ok(Some((stream.consume_char(byte_ofs, ch), ch)))
        } else {
            Ok(None)
        }
    }
}

//ip ParserInput for AbcTokenStream
impl<'a> ParserInputStream<AbcTokenStream<'a>> for AbcTokenStream<'a> {
    //
    fn get_token(&self) -> Result<Option<(Self, char)>, AbcTokenStreamError> {
        Ok(self
            .stream
            .parse(&[Self::parse_char_fn])?
            .map(|(stream, t)| (Self { stream }, t)))
    }
}

//a Tests
//fi test_me
/// 'static is required for the stream even though it will have a
/// lifetime that is for a single parse If 'parser is used then it
/// will force the parser to have a lifetime that matches the token
/// stream, whereas the parser must outlive it.
///
/// If a 'for <'stream>' is used then Rust cannot work out the
/// lifetimes required for the individual subparsers (there will be
/// 'type is more general' errors for closures where Rust deduces
/// various different lifetimes for the subparsers compared to what it
/// infers is needed. Often you will get lifetime '_ is more general
/// than lifetime '_.)
///
/// 'static requires an unsafe cast in the use of the parser; this can
/// only be rendered safe if the use of the parser is bounded within a
/// function which can 'own' the results of the parse. At the point at
/// which the results of the parse are dropped, the stream which is
/// being parsed can be dropped.
///
/// Hence a parse should be wrapped in a borrow of the parser with a
/// borrow of the text to be parsed; parsing can then take place, and
/// results of the parse used, and (provided the parse results do not
/// use the input stream borrow) the results can the be returned and
/// the operation will be clean.
///
/// If the parse results are to require a borrow of the text stream
/// then the caller must handle the safety. Perhaps a wrapping
/// structure that contains the input stream *and* the parsed results
/// would be sufficient?
type AbcParserFn<'parser, 'stream, R> =
    dyn Fn(AbcTokenStream<'stream>) -> ParseFnResult<AbcTokenStream<'stream>, R> + 'parser;
struct AbcParser<'parser> {
    at_least_one_a: Box<AbcParserFn<'parser, 'static, usize>>,
    some_bs: Box<AbcParserFn<'parser, 'static, usize>>,
    at_least_one_c: Box<AbcParserFn<'parser, 'static, usize>>,
    grammar1: Box<AbcParserFn<'parser, 'static, (usize, usize, usize)>>,
    grammar2: Box<AbcParserFn<'parser, 'static, (usize, usize, usize)>>,
    either_grammar: Box<AbcParserFn<'parser, 'static, (usize, usize, usize)>>,
    _pin: std::marker::PhantomPinned,
}
macro_rules! abc_pref {
    ($R:ty, $p:ident, $e:ident, $l:lifetime) => {
        unsafe {
            std::mem::transmute::<
                &Box<AbcParserFn<$l, 'static, $R>>,
                &Box<AbcParserFn<'_, 'static, $R>>,
            >(&$p.$e)
        }
    };
}
macro_rules! abc_pset {
    ($p:ident, $e:ident, $f:expr) => {
        let mut_p = std::pin::Pin::as_mut(&mut $p);
        unsafe {
            std::pin::Pin::get_unchecked_mut(mut_p).$e = Box::new($f);
        }
    };
}
impl<'parser> AbcParser<'parser> {
    fn new() -> std::pin::Pin<Box<Self>> {
        let at_least_one_a = Box::new(parser_fn::count_of(|t| (t == 'a'), 1..1000));
        let some_bs = Box::new(parser_fn::count_of(|t| (t == 'b'), 0..1000));
        let at_least_one_c = Box::new(parser_fn::count_of(|t| (t == 'c'), 1..1000));
        let grammar1 = Box::new(parser_fn::success(|| (0_usize, 0_usize, 0_usize)));
        let grammar2 = Box::new(parser_fn::success(|| (0_usize, 0_usize, 0_usize)));
        let either_grammar = Box::new(parser_fn::success(|| (0_usize, 0_usize, 0_usize)));
        let mut parser = Box::pin(AbcParser {
            at_least_one_a,
            some_bs,
            at_least_one_c,
            grammar1,
            grammar2,
            either_grammar,
            _pin: std::marker::PhantomPinned,
        });
        let at_least_one_a = abc_pref!(usize, parser, at_least_one_a, 'parser);
        let at_least_one_c = abc_pref!(usize, parser, at_least_one_c, 'parser);
        let some_bs = abc_pref!(usize, parser, some_bs, 'parser);
        abc_pset!(
            parser,
            grammar1,
            parser_fn::tuple3_ref(at_least_one_a, some_bs, at_least_one_c)
        );
        abc_pset!(
            parser,
            grammar2,
            parser_fn::tuple3_ref(at_least_one_c, some_bs, at_least_one_a)
        );
        let grammar1 = abc_pref!((usize, usize, usize), parser, grammar1, 'parser);
        let grammar2 = abc_pref!((usize, usize, usize), parser, grammar1, 'parser);
        abc_pset!(
            parser,
            either_grammar,
            parser_fn::first_of_n_dyn_ref_else([grammar1, grammar2], || {
                AbcTokenStreamError::Other("Matched neither grammar".to_string())
            })
        );
        parser
    }
    fn do_test(&self, s: &str) {
        let ps = unsafe { std::mem::transmute::<&str, &'static str>(s) };
        let stream = TextStreamSpan::new(ps);
        let abcs = AbcTokenStream { stream };

        println!("{:?}", (*self.at_least_one_a)(abcs));
        println!("{:?}", (*self.grammar1)(abcs));
        println!("{:?}", (*self.grammar2)(abcs));
        println!("{:?}", (*self.either_grammar)(abcs));
        drop(stream);
    }
}

fn do_test(abc_parser: &AbcParser, s: &str) {
    abc_parser.do_test(s);
}

#[test]
fn test_me() {
    let abc_parser = AbcParser::new();
    {
        do_test(&abc_parser, "abc");
        do_test(&abc_parser, "abbbbbc");
        do_test(&abc_parser, "cba");
        do_test(&abc_parser, &format!("cba"));
    }
    // assert!(false);
}
