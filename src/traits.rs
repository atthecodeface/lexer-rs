use crate::StreamCharSpan;
//a PosnIn traits
//tt PosnInStream
/// Trait for location within a stream
///
/// This base trait is used to enable tracking the position of a token
/// parser within a stream in a manner that is useful for
/// human-readable error messages
///
/// A simple implementation can be NULL, if the position is not
/// critical for error messages for the token parser - for example,
/// parsing a simple string in a test.
///
/// For a single file implementation see [crate::LineColumn]
pub trait PosnInStream:
    Sized + std::fmt::Debug + Copy + std::default::Default + PartialEq + Eq + std::hash::Hash
{
    fn advance_cols(self, _byte_ofs: usize, _num_chars: usize) -> Self {
        self
    }
    fn advance_line(self, _byte_ofs: usize) -> Self {
        self
    }
    fn error_fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self, fmt)
    }
}

//ip PosnInStream for ()
impl PosnInStream for () {}

//tt PosnInCharStream
/// Trait for location within a character stream
///
/// This tracks a byte offset within the stream so that strings can be
/// retrieved from the stream. Byte offsets *must* always be on UTF8
/// boundaries.
pub trait PosnInCharStream: PosnInStream {
    fn byte_ofs(&self) -> usize;
}

//ip PosnInStream for usize
impl PosnInStream for usize {
    fn advance_cols(self, byte_ofs: usize, _num_chars: usize) -> Self {
        self + byte_ofs
    }
    fn advance_line(self, byte_ofs: usize) -> Self {
        self + byte_ofs
    }
}

//ip PosnInCharStream for usize
impl PosnInCharStream for usize {
    fn byte_ofs(&self) -> usize {
        *self
    }
}

//a Lexer etc
//tt Lexer
pub trait Lexer: std::fmt::Debug {
    type Token: Sized + std::fmt::Debug + Copy;
    type State: Sized + Copy + std::fmt::Debug;
    type Error: LexerError<Self::State>;
    fn parse(&self, state: Self::State, parsers: &[LexerParseFn<Self>]) -> LexerParseResult<Self::State, Self::Token, Self::Error>;
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
// pub type LexerParseResult<L> =
//     Result<Option<(<L as Lexer>::State, <L as Lexer>::Token)>, <L as Lexer>::Error>;
pub type LexerParseResult<S, T, E> =
    Result<Option<(S, T)>, E>;

//tp LexerParseFn
pub type LexerParseFn<L> = fn(lexer: &L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error>;

//tt LexerError
/// A trait required of an error - a char that does not match any
/// token parser rust return an error, and this trait requires that
/// such an error be provided
///
/// It might be nice to have this take the [Lexer] too, but then there
/// is a cycle in that Lexer::Error will in general depend on Lexer
/// which depends on Lexer::Error...
pub trait LexerError<P> : Sized + std::error::Error
{
    fn failed_to_parse(state: P, ch: char) -> Self;
}
