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

//a Span
//tp Span
/// A span within a str
#[derive(Debug, Copy, Clone)]
pub struct Span<P>
where
    P: PosnInStream,
{
    start: Pos<P>,
    end: Pos<P>,
}

//ip Span
impl<P> Span<P>
where
    P: PosnInStream,
{
    //fp new
    /// Create a new [Span]
    pub fn new(start: Pos<P>, end: Pos<P>) -> Self {
        Self { start, end }
    }

    //mp byte_range
    /// Get the range between the two positions of this [Span]
    pub fn byte_range(&self) -> Range<usize> {
        let start = self.start.byte_ofs;
        let end = self.end.byte_ofs;
        Range { start, end }
    }
}
