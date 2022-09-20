//a Imports
use crate::{PosnInCharStream, StreamCharSpan, PosnInStream};

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

//a Lexer
//tt Lexer
/// The [Lexer] trait is provided by stream types that support parsing
/// into tokens.
///
/// The trait itself requires:
///
/// * a token type that the [Lexer] will produce
///
/// * a stream state (often just a byte offset) that can be tracked
///   during parsing
///
/// * an error type that suports [LexerError] so that the lexer can
///   generate a failure should a token parse fail
///
/// The [Lexer] will parse its stream provided to it by matching data in
/// the stream to tokens using parser functions. Such functions are
/// invoked with a reference to the stream being parsed, the stream
/// state, and the next character in the stream (the one pointed to by
/// the stream state).
///
/// The signature is:
///
/// ```ignore
///    fn parse(stream: &LexerOfStr<P, T, E>, pos:P, ch:char) ->
///               LexerParseResult<P, T, E>
/// ```
///
/// where
///
/// ```ignore
///    LexerParseResult<P, T, E> = Result<Option<P, T>, E>
/// ```
///
/// Parsing functions examine the character they are given, and
/// possibly more characters by accessing the stream using the provide
/// state. If they match, they return an Ok result with the token they
/// have parsed to, *and* an updated state which is *beyond* the
/// matched token.
///
/// If the parser function mismatches then it returns an Ok result of None
///
/// If the parser function hits a fatal error (for example, a stream
/// indicates a network disconnection) then it must return an Err with
/// the appropriate error (of its provided Error type).
///
/// Parser functions are provided to the [Lexer] as an array of Box dyn
/// functions, such as:
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
///
/// This trait is provided in part to group the types for a lexical
/// parser together, enabling simpler type inference and less
/// turbofish syntax in clients of the lexical analysis.
pub trait Lexer: std::fmt::Debug {

    /// The Token type is the type of the token to be returned by the
    /// Lexer; it is used as part of the result of the [Lexer] parse
    /// functions.
    type Token: Sized + std::fmt::Debug;

    /// The State of the stream that is used and returned by the parse
    /// functions; it must be copy as it is replicated constantly
    /// throughout the parsing process.
    ///
    /// This can be a [crate::StreamCharPos]
    type State: Sized + Copy + std::fmt::Debug + Default;

    /// The error type returned by the parser functions in the lexical analyzer
    type Error: LexerError<Self::State>;

    /// This attempts to parse the next token found at the state of
    /// the [Lexer] stream, by applying the parsers in order.
    ///
    /// An error is returned if the token cannot be parsed
    fn parse<'a>(
        &'a self,
        state: Self::State,
        parsers: &[BoxDynLexerParseFn<'a, Self>],
    ) -> LexerParseResult<Self::State, Self::Token, Self::Error>;

    /// This creates an iterator over all of the tokens in the [Lexer]
    /// stream, by applying the parsers in order at the current stream
    /// position whenever the 'next' method is invoked.
    ///
    /// The iterator returns None when the end of stream is reached,
    /// otherwise it returns a result of the token or an error,
    /// depending on the success of the parsers.
    fn iter<'iter>(
        &'iter self,
        parsers: &'iter [BoxDynLexerParseFn<'iter, Self>],
    ) -> Box<dyn Iterator<Item = Result<Self::Token, Self::Error>> + 'iter>;
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
