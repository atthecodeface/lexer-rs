//a PosnInStream
/// Trait for location within a stream
///
/// This trait is actual type can be anything, such
/// as 
pub trait PosnInStream:
    Sized + std::fmt::Debug + Copy + std::default::Default + PartialEq + Eq + std::hash::Hash
{
    fn advance_cols(&mut self, _num_chars: usize) {}
    fn advance_line(&mut self) {}
}

//ip PosnInStream for simple types
impl PosnInStream for () {}
impl PosnInStream for u8 {}

//tt TokenType
/// The traits required of a token
pub trait TokenType: Sized + std::fmt::Debug + Copy {}

//ip TokenType for char and u*
impl TokenType for char {}
impl TokenType for u8 {}
impl TokenType for u16 {}
impl TokenType for u32 {}
impl TokenType for usize {}

//tt TokenTypeError
/// A trait required of an error - a char that does not match any
/// token parser rust return an error, and this trait requires that
/// such an error be provided
use crate::lexer::Pos;
pub trait TokenTypeError<P: PosnInStream>: Sized + std::error::Error {
    fn failed_to_parse(ch: char, pos: Pos<P>) -> Self;
}

