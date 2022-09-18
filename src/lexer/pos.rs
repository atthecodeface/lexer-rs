//a Imports
use crate::{PosnInStream, PosnInCharStream};

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
    fn advance_cols(mut self, byte_ofs: usize, num_chars: usize) -> Self {
        self.byte_ofs = byte_ofs;
        self.pos.advance_cols(byte_ofs, num_chars);
        self
    }
    fn advance_line(mut self, byte_ofs: usize) -> Self {
        self.byte_ofs = byte_ofs;
        self.pos.advance_line(byte_ofs);
        self
    }
}

//ip PosnInStream for StreamCharPos
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
    P: PosnInStream
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.pos.error_fmt(fmt)
    }
}

