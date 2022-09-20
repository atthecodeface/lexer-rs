//a Imports
use crate::{PosnInCharStream, PosnInStream};

//a StreamCharPos
//tp StreamCharPos
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StreamCharPos<P>
where
    P: PosnInStream,
{
    byte_ofs: usize,
    pos: P,
}

//ip StreamCharPos
impl<P> StreamCharPos<P>
where
    P: PosnInStream,
{
    pub fn pos(&self) -> P {
        self.pos
    }
}

//ip PosnInStream for StreamCharPos
impl<P> PosnInStream for StreamCharPos<P>
where
    P: PosnInStream,
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
    P: PosnInStream,
{
    fn byte_ofs(&self) -> usize {
        self.byte_ofs
    }
}

//ip Display for StreamCharPos
impl<P> std::fmt::Display for StreamCharPos<P>
where
    P: PosnInStream,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.pos.error_fmt(fmt)
    }
}
