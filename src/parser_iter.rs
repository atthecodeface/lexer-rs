//a Imports
use crate::{Lexer, LexerParseFn};

//a ParserIterator
//tp ParserIterator
/// An iterator over a Lexer presenting the parsed Tokens from it
use crate::{LexerParseResult, BoxDynLexerPasrseFn};

pub struct ParserIterator<'a, L>
where
    L: Lexer,
{
    lexer: &'a L,
    state: L::State,
    parsers: &'a [BoxDynLexerPasrseFn<'a, L>],
}

//ip ParserIterator
impl<'a, L> ParserIterator<'a, L>
where
    L: Lexer,
    // F : std::ops::Deref<Target = dyn Fn(&'a L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error> + 'a>,
{
    /// Create a new token stream iterator to parse a string and deliver tokens
    pub fn new(lexer: &'a L, state: L::State, parsers: &'a [BoxDynLexerPasrseFn<'a, L>]) -> Self {
        Self {
            lexer,
            state,
            parsers,
        }
    }
}

//ip Iterator for ParserIterator
impl<'a, L> Iterator for ParserIterator<'a, L>
where
    L: Lexer,
//    F : std::ops::Deref<Target = dyn Fn(&'a L, <L as Lexer>::State, char) -> LexerParseResult<<L as Lexer>::State, <L as Lexer>::Token, <L as Lexer>::Error> + 'a>,
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
