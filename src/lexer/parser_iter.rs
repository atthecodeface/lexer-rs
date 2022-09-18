//a Imports
use crate::{Lexer, LexerParseFn};

//a ParserIterator
//tp ParserIterator
/// An iterator over a Lexer presenting the parsed Tokens from it
pub struct ParserIterator<'a, L>
where
    L: Lexer
{
    lexer: &'a L,
    state: L::State,
    parsers: &'a [LexerParseFn<L>],
}

//ip ParserIterator
impl<'a, L> ParserIterator<'a, L>
where
    L: Lexer,
{
    /// Create a new token stream iterator to parse a string and deliver tokens
    pub fn new(lexer: &'a L, state:L::State, parsers: &'a [LexerParseFn<L>]) -> Self {
        Self { lexer, state, parsers }
    }
}

//ip Iterator for ParserIterator
impl<'a, L> Iterator for ParserIterator<'a, L> 
where
    L: Lexer,
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
