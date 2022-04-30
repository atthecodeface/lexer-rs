//a Imports
use lexer::parser_fn;
use lexer::TokenParseError;
use lexer::{Parser, ParserFnInput, ParserFnResult};
use lexer::{TextPos, TextStream, TextStreamSpan};

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
    type Error = TokenParseError<Pos>;
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
    fn get_token(&self) -> Result<Option<(Self, char)>, TokenParseError<Pos>> {
        Ok(self
            .stream
            .parse(&[Self::parse_char_fn])?
            .map(|(stream, t)| (Self { stream }, t)))
    }
}

//a Tests
//fi test_me
#[test]
fn test_me() {
    let a = r##"aabbbc"##;

    let text = TextStream::new(a);
    let stream = text.as_span();
    let abcs = AbcTokenStream { stream };

    let is_a = parser_fn::map_token(|t| if t == 'a' { Some('a') } else { None });
    let at_least_one_a = parser_fn::match_count(|t| (t == 'a'), 1..1000);
    let some_bs = parser_fn::match_count(|t| (t == 'b'), 0..1000);
    let at_least_one_c = parser_fn::match_count(|t| (t == 'c'), 1..1000);
    let grammar1 = parser_fn::tuple3_ref(&at_least_one_a, &some_bs, &at_least_one_c);
    let grammar2 = parser_fn::tuple3_ref(&at_least_one_c, &some_bs, &at_least_one_a);
    // let either_grammar = parser_fn::first_of_2_ref(&grammar2, &grammar1);
    let grammars: [&dyn Fn(_) -> _; 2] = [&grammar2, &grammar1];
    let either_grammar = parser_fn::first_of_n_dyn_ref(grammars);
    println!("{:?}", is_a(abcs));
    println!("{:?}", at_least_one_a(abcs));
    println!("{:?}", grammar1(abcs));
    println!("{:?}", grammar2(abcs));
    println!("{:?}", either_grammar(abcs));
    assert!(false);
}
