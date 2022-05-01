//a Imports
use lexer::parser_fn;
use lexer::{Parser, ParserFnInput, ParserFnResult};
use lexer::{TextPos, TextStream, TextStreamSpan};
use lexer::{TokenParseError, TokenTypeError};

//a Pos
//tp Pos
#[derive(Debug, Clone, Copy, Default)]
struct Pos(());

//ip TextPos of Pos
impl TextPos for Pos {}

//ip Display for Pos
impl std::fmt::Display for Pos {
    fn fmt(&self, _fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

//a AbcTokenStream
//tp AbcTokenStreamError
#[derive(Debug)]
enum AbcTokenStreamError {
    Token(TokenParseError<Pos>),
    Other(String),
}
impl std::fmt::Display for AbcTokenStreamError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
impl std::error::Error for AbcTokenStreamError {}
impl TokenTypeError<Pos> for AbcTokenStreamError {
    fn failed_to_parse(_: char, _: TextStreamSpan<'_, Pos>) -> Self {
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

//ip Parser for AbcTokenStream
impl<'a> Parser<'a> for AbcTokenStream<'a> {
    type Token = char;
    type Pos = Pos;
    type Error = AbcTokenStreamError;
    type Input = AbcTokenStream<'a>;
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

//ip ParserFnInput for AbcTokenStream
impl<'a> ParserFnInput<'a, AbcTokenStream<'a>> for AbcTokenStream<'a> {
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
type AbcParserFn<'a, R> = dyn Fn(AbcTokenStream<'a>) -> ParserFnResult<'a, AbcTokenStream<'a>, R> + 'a;
struct AbcParser<'a> {
    at_least_one_a: Box<AbcParserFn<'a, usize>>,
    some_bs: Box<AbcParserFn<'a, usize>>,
    at_least_one_c: Box<AbcParserFn<'a, usize>>,
    grammar1: Box<AbcParserFn<'a, (usize, usize, usize)>>,
    grammar2: Box<AbcParserFn<'a, (usize, usize, usize)>>,
    either_grammar: Box<AbcParserFn<'a, (usize, usize, usize)>>,
    _pin : std::marker::PhantomPinned,
}
impl <'a> AbcParser<'a> {
    fn new() -> std::pin::Pin<Box<Self>> {
        let at_least_one_a = Box::new(parser_fn::match_count(|t| (t == 'a'), 1..1000));
        let some_bs = Box::new(parser_fn::match_count(|t| (t == 'b'), 0..1000));
        let at_least_one_c = Box::new(parser_fn::match_count(|t| (t == 'c'), 1..1000));
        let grammar1 = Box::new(parser_fn::tuple3(parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize)));
        let grammar2 = Box::new(parser_fn::tuple3(parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize)));
        let either_grammar = Box::new(parser_fn::tuple3(parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize),
                                                        parser_fn::success(0_usize)));
        let mut parser = Box::pin(AbcParser {
            at_least_one_a,
            some_bs,
            at_least_one_c,
            grammar1,
            grammar2,
            either_grammar,
            _pin : std::marker::PhantomPinned,
        });
        let at_least_one_a = unsafe {std::mem::transmute::<&Box<AbcParserFn<'a, usize>>, &Box<AbcParserFn<'_, usize>>>(&parser.at_least_one_a)};
        let at_least_one_c = unsafe {std::mem::transmute::<&Box<AbcParserFn<'a, usize>>, &Box<AbcParserFn<'_, usize>>>(&parser.at_least_one_c)};
        let some_bs = unsafe {std::mem::transmute::<&Box<AbcParserFn<'a, usize>>, &Box<AbcParserFn<'a, usize>>>(&parser.some_bs)};
        let mut_p = std::pin::Pin::as_mut(&mut parser);
        unsafe {
            std::pin::Pin::get_unchecked_mut(mut_p).grammar1 = Box::new(parser_fn::tuple3_ref(at_least_one_a, some_bs, at_least_one_c));
        }
        let mut_p = std::pin::Pin::as_mut(&mut parser);
        unsafe {
            std::pin::Pin::get_unchecked_mut(mut_p).grammar2 = Box::new(parser_fn::tuple3_ref(at_least_one_c, some_bs, at_least_one_a));
        }
        let grammar1 = unsafe {std::mem::transmute::<&Box<AbcParserFn<'a, (usize, usize, usize)>>, &Box<AbcParserFn<'_, (usize, usize, usize)>>>(&parser.grammar1)};
        let grammar2 = unsafe {std::mem::transmute::<&Box<AbcParserFn<'a, (usize, usize, usize)>>, &Box<AbcParserFn<'_, (usize, usize, usize)>>>(&parser.grammar2)};
        let grammars: [&dyn Fn(_) -> _; 2] = [grammar2, grammar1];
        let mut_p = std::pin::Pin::as_mut(&mut parser);
        unsafe {
            std::pin::Pin::get_unchecked_mut(mut_p).either_grammar = Box::new( parser_fn::first_of_n_dyn_ref_else(grammars, || {
                AbcTokenStreamError::Other("Matched neither grammar".to_string())}));
        }
        parser
    }
}

fn do_test<'a, 'parser>(abc_parser:&'a AbcParser<'parser>, a:&'a str) {
    let text = TextStream::new(a);
    let stream = text.as_span();
    let abcs = AbcTokenStream { stream };

    println!("{:?}", (*abc_parser.at_least_one_a)(abcs));
    println!("{:?}", (*abc_parser.grammar1)(abcs));
    println!("{:?}", (*abc_parser.grammar2)(abcs));
    println!("{:?}", (*abc_parser.either_grammar)(abcs));
    drop(stream);
    drop(text);
}
#[test]
fn test_me() {
    let abc_parser = AbcParser::new();
    do_test(&abc_parser, "abc");
    do_test(&abc_parser, "abbbbbc");
    do_test(&abc_parser, "cba");
    assert!(false);
}
