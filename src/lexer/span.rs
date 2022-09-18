//a Imports
use std::ops::Range;

use crate::PosnInCharStream;

//a StreamCharSpan
//tp StreamCharSpan
/// A span within a str
#[derive(Debug, Clone, Copy)]
pub struct StreamCharSpan<P>
where
    P: PosnInCharStream,
{
    start: P,
    end: P,
}

//ip StreamCharSpan
impl<P> StreamCharSpan<P>
where
    P: PosnInCharStream,
{
    //fp new
    /// Create a new [StreamCharSpan]
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }

    //ap start
    /// Get the start of the span
    pub fn start(&self) -> &P {
        &self.start
    }

    //ap end
    /// Get the end of the span
    pub fn end(&self) -> &P {
        &self.end
    }

    //mp byte_range
    /// Get the range between the two positions of this [StreamCharSpan]
    pub fn byte_range(&self) -> Range<usize> {
        let start = self.start.byte_ofs();
        let end = self.end.byte_ofs();
        Range { start, end }
    }
}
