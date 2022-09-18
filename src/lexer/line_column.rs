//a Imports
use std::ops::Range;

use crate::PosnInStream;

//a LineColumn
//tp LineColumn
/// A line + column within a text stream
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LineColumn {
    line: usize,
    column: usize,
}

//ip Default for LineColumn
impl std::default::Default for LineColumn {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

//ip Display for LineColumn
impl std::fmt::Display for LineColumn {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "line {} column {}", self.line, self.column)
    }
}

//ip PosnInStream for LineColumn
impl PosnInStream for LineColumn {
    fn advance_cols(&mut self, num_chars: usize) {
        self.column += num_chars;
    }
    fn advance_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }
}

