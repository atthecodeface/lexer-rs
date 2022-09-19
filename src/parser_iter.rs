//a Imports
use crate::{Lexer, LexerParseFn};

//a ParserIterator
//tp ParserIterator
/// An iterator over a Lexer presenting the parsed Tokens from it
use crate::LexerParseResult;
pub struct ParserIterator<'a, L, F>
where
    L: Lexer,
    F : std::ops::Deref<Target = dyn Fn(&'a L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error> + 'a>,
{
    lexer: &'a L,
    state: L::State,
    parsers: &'a [F],
}

//ip ParserIterator
impl<'a, L, F> ParserIterator<'a, L, F>
where
    L: Lexer,
    F : std::ops::Deref<Target = dyn Fn(&'a L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error> + 'a>,
{
    /// Create a new token stream iterator to parse a string and deliver tokens
    pub fn new(lexer: &'a L, state: L::State, parsers: &'a [F]) -> Self {
        Self {
            lexer,
            state,
            parsers,
        }
    }
}

//ip Iterator for ParserIterator
impl<'a, L, F> Iterator for ParserIterator<'a, L, F>
where
    L: Lexer,
    F : std::ops::Deref<Target = dyn Fn(&'a L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error> + 'a>,
{
    type Item = Result<L::Token, L::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.parse(self.state, self.parsers) {
            Err(e) => Some(Err(e)),
            Ok(Some((state, token))) => {
                self.state = state;
                Some(Ok(token))
            }
            _ => None,
        }
    }
}
