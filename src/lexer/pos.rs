//a Imports
use std::ops::Range;

//a TextPos
/// Trait for location within a file
pub trait TextPos:
    Sized + std::fmt::Debug + std::fmt::Display + Copy + std::default::Default + PartialEq + Eq + std::hash::Hash
{
    fn advance_cols(&mut self, _num_chars: usize) {}
    fn advance_line(&mut self) {}
}

//ip TextPos for u8
impl TextPos for u8 {}

//a LineCol
//tp LineCol
/// A line + column within a text stream
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LineCol {
    line: usize,
    column: usize,
}

//ip Default for LineCol
impl std::default::Default for LineCol {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

//ip Display for LineCol
impl std::fmt::Display for LineCol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "line {} column {}", self.line, self.column)
    }
}

//ip TextPos for LineCol
impl TextPos for LineCol {
    fn advance_cols(&mut self, num_chars: usize) {
        self.column += num_chars;
    }
    fn advance_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }
}

//a Pos
//tp Pos
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos<P>
where
    P: TextPos,
{
    byte_ofs: usize,
    pos: P,
}

//ip Pos
impl<P> Pos<P>
where
    P: TextPos,
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
    P: TextPos,
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
    P: TextPos,
{
    start: Pos<P>,
    end: Pos<P>,
}

//ip Span
impl<P> Span<P>
where
    P: TextPos,
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
