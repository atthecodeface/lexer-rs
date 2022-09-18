//a Imports
use std::ops::Range;

use crate::PosnInStream;

//a Pos
//tp Pos
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos<P>
where
    P: PosnInStream,
{
    byte_ofs: usize,
    pos: P,
}

//ip Pos
impl<P> Pos<P>
where
    P: PosnInStream,
{
    pub fn pos(&self) -> P {
        self.pos
    }
    pub fn advance_cols(mut self, byte_ofs: usize, num_chars: usize) -> Self {
        self.byte_ofs = byte_ofs;
        self.pos.advance_cols(num_chars);
        self
    }
    pub fn advance_line(mut self, byte_ofs: usize) -> Self {
        self.byte_ofs = byte_ofs;
        self.pos.advance_line();
        self
    }
    pub fn byte_ofs(&self) -> usize {
        self.byte_ofs
    }
}

//ip Display for Pos
impl<P> std::fmt::Display for Pos<P>
where
    P: PosnInStream + std::fmt::Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&self.pos, fmt)
    }
}

