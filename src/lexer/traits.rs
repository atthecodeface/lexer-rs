//a Imports
use crate::{PosnInCharStream, StreamCharSpan};

//a LexerError
//tt LexerError
/// A trait required of an error within a Lexer - a char that does not
/// match any token parser rust return an error, and this trait
/// requires that such an error be provided
///
/// It might be nice to have this take the [Lexer] too, but then there
/// is a cycle in that Lexer::Error will in general depend on Lexer
/// which depends on Lexer::Error... This breaks code (and the compiler
/// tends to hang forever)
pub trait LexerError<P>: Sized + std::error::Error {
    /// Return an error indicating that a bad character (could not be
    /// matched for a token) has occurred at the position indicated by
    /// the state
    fn failed_to_parse(state: P, ch: char) -> Self;
}

//a Lexer, LexerOfChar
//tt Lexer
pub trait Lexer: std::fmt::Debug {
    type Token: Sized + std::fmt::Debug + Copy;
    type State: Sized + Copy + std::fmt::Debug + Default;
    type Error: LexerError<Self::State>;
    fn parse<'a>(
        &'a self,
        state: Self::State,
        parsers: &[BoxDynLexerParseFn<'a, Self>],
    ) -> LexerParseResult<Self::State, Self::Token, Self::Error>;
    fn iter<'iter>(
        &'iter self,
        parsers: &'iter [BoxDynLexerParseFn<'iter, Self>],
    ) -> Box<dyn Iterator<Item = Result<Self::Token, Self::Error>> + 'iter>;
}

//tt LexerOfChar
// Requires Lexer::State : PosnInCharStream>
pub trait LexerOfChar: Lexer {
    fn do_while<F: Fn(usize, char) -> bool>(
        &self,
        state: Self::State,
        ch: char,
        f: &F,
    ) -> (Self::State, Option<(Self::State, usize)>);
    fn range_as_bytes(&self, ofs: usize, n: usize) -> &[u8];
    fn get_text_span(&self, span: &StreamCharSpan<Self::State>) -> &str
    where
        <Self as Lexer>::State: PosnInCharStream;
    fn get_text(&self, start: Self::State, end: Self::State) -> &str;
    fn consume_ascii_str(&self, state: Self::State, s: &str) -> Self::State;
    fn consume_char(&self, state: Self::State, ch: char) -> Self::State;
    fn consumed_newline(&self, state: Self::State, num_bytes: usize) -> Self::State;
    fn consumed_chars(&self, state: Self::State, num_bytes: usize, num_chars: usize)
        -> Self::State;
    fn matches(&self, state: &Self::State, s: &str) -> bool;
    fn matches_bytes(&self, state: &Self::State, s: &[u8]) -> bool;
    fn peek_at(&self, state: &Self::State) -> Option<char>;
}

//tp LexerParseResult
/// The return value for a Lexer parse function
///
/// This *could* have been defined as:
///
///    pub type LexerParseResult<L:Lexer>
///      = Result<Option<(<L as Lexer>::State, <L as Lexer>::Token)>, <L as Lexer>::Error>;
///
/// But then clients that have their type L with a lifetime (which is common) would have a parse
/// result that must be indicated by a lifetime, where the actual result *does not*.
///
/// This causes problems for clients
pub type LexerParseResult<S, T, E> = Result<Option<(S, T)>, E>;

//tp LexerParseFn
/// The type of a parse function
pub type LexerParseFn<L> =
    fn(
        lexer: &L,
        <L as Lexer>::State,
        char,
    ) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error>;

//tp BoxDynLexerParseFn
/// The type of a parse function, when Boxed as a dyn trait
///
/// This type can be used in arrays/slices to allow a Lexer to run
/// through a list of possible token parsers such as:
///
/// ```ignore
///       let parsers = [
///            Box::new(parse_char_fn) as BoxDynLexerParseFn<OurLexer>
///            Box::new(parse_value_fn),
///            Box::new(parse_whitespace_fn),
///        ];
/// ```
///
/// Note that the use of 'as Box...' is required, as without it type
/// inference will kick in on the Box::new() to infer parse_char_fn as
/// a precise type, whereas the more generic dyn Fn is what is required.
pub type BoxDynLexerParseFn<'a, L> = Box<
    dyn for<'call> Fn(
            &'call L,
            <L as Lexer>::State,
            char,
        ) -> LexerParseResult<
            <L as Lexer>::State,
            <L as Lexer>::Token,
            <L as Lexer>::Error,
        > + 'a,
>;
