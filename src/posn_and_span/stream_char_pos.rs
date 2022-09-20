//a Imports
use crate::{PosnInCharStream, UserPosn};

//a StreamCharPos
//tp StreamCharPos
/// This provides the byte offset of a character within a stream, with
/// an associated position that might also accurately provide line and
/// column numbers of the position
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StreamCharPos<P>
where
    P: UserPosn,
{
    byte_ofs: usize,
    pos: P,
}

//ip StreamCharPos
impl<P> StreamCharPos<P>
where
    P: UserPosn,
{
    /// Get the descriptive position that [Self] includes; if that
    /// type provides them accurately, this can give the line number
    /// and column number of the position
    pub fn pos(&self) -> P {
        self.pos
    }
}

//ip UserPosn for StreamCharPos
impl<P> UserPosn for StreamCharPos<P>
where
    P: UserPosn,
{
    fn advance_cols(mut self, num_bytes: usize, num_chars: usize) -> Self {
        self.byte_ofs += num_bytes;
        self.pos = self.pos.advance_cols(num_bytes, num_chars);
        self
    }
    fn advance_line(mut self, num_bytes: usize) -> Self {
        self.byte_ofs += num_bytes;
        self.pos = self.pos.advance_line(num_bytes);
        self
    }
    fn line(&self) -> usize {
        self.pos.line()
    }

    fn column(&self) -> usize {
        self.pos.column()
    }
    fn error_fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.pos.error_fmt(fmt)
    }
}

//ip PosnInCharStream for StreamCharPos
impl<P> PosnInCharStream for StreamCharPos<P>
where
    P: UserPosn,
{
    fn byte_ofs(&self) -> usize {
        self.byte_ofs
    }
}

//ip Display for StreamCharPos
impl<P> std::fmt::Display for StreamCharPos<P>
where
    P: UserPosn,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.pos.error_fmt(fmt)
    }
}
