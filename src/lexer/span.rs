//a Imports
use std::ops::Range;

use crate::{PosnInStream, Pos};

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
        let start = self.start.byte_ofs();
        let end = self.end.byte_ofs();
        Range { start, end }
    }
}
