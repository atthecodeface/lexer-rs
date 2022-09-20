//a Imports
use crate::UserPosn;

//a LineColumn
//tp LineColumn
/// A line and column within a text stream
///
/// This provides the [UserPosn] trait, which provides methods to
/// retrieve the line and column values of the state.
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

//ip UserPosn for LineColumn
impl UserPosn for LineColumn {
    fn line(&self) -> usize {
        self.line
    }

    fn column(&self) -> usize {
        self.column
    }
    fn advance_cols(mut self, _: usize, num_chars: usize) -> Self {
        self.column += num_chars;
        self
    }
    fn advance_line(mut self, _: usize) -> Self {
        self.column = 1;
        self.line += 1;
        self
    }
    fn error_fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "line {} column {}", self.line, self.column)
    }
}
