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

