//a Imports
use crate::UserPosn;

//a FmtContext
//tt FmtContext
/// This trait is provided by types that wish to support context for
/// (e.g.) error messages
///
/// It requires the type to have the ability to map from a line number
/// to a position within the file/stream/text of the type, and to
/// provide the length of any specific line nummber.e
///
/// With those supplied methods, the trait provides the 'fmt_context'
/// method, which outputs to a formatter (which can be an &mut String
/// even) the lines of the text ahead of a provided span of start and
/// end positions within the stream.
///
/// Currently the format of the context is fixed - the number of lines
/// ahead is fixed a a maximum of four, the lines are always numbered
/// with aa line number of up to 4 digits, and so on.
pub trait FmtContext<P> {
    /// Return the length of the specified line
    fn line_length(&self, line: usize) -> usize;

    /// Format the line of text (potentially with coloring and so on).
    ///
    /// This formatting must preserve the columnn numbers of characters
    /// if context markers are to line up correctly
    fn fmt_line(&self, f: &mut dyn std::fmt::Write, line: usize) -> std::fmt::Result;

    /// Format a line of text with highlight on certain columns
    fn fmt_context_single_line(
        &self,
        f: &mut dyn std::fmt::Write,
        start: &P,
        num_cols: usize, // number of columns to highlight
    ) -> std::fmt::Result
    where
        P: UserPosn,
    {
        let line = start.line();
        let first_col = start.column();
        if line > 1 {
            write!(f, "    |  ")?;
            self.fmt_line(f, line - 1)?;
            writeln!(f)?;
        }
        write!(f, "{:4}|  ", line)?;
        self.fmt_line(f, line)?;
        writeln!(f)?;
        write!(f, "    |  ")?;
        for _ in 1..(first_col) {
            f.write_char(' ')?;
        }
        if num_cols == 0 {
            f.write_char('^')?;
        } else {
            for _ in 0..num_cols {
                f.write_char('^')?;
            }
        }
        writeln!(f)?;
        writeln!(f, "    |  ")
    }

    /// Format multiple lines of text, highlighting certain lines
    fn fmt_context_multiple_lines(
        &self,
        f: &mut dyn std::fmt::Write,
        start: &P,
        end: &P,
    ) -> std::fmt::Result
    where
        P: UserPosn,
    {
        let first_line = start.line();
        let first_line = if first_line > 1 {
            first_line - 1
        } else {
            first_line
        };
        let last_line = end.line() + {
            if end.column() == 0 {
                0
            } else {
                1
            }
        };
        // Change to a Range
        let num_lines = {
            if last_line <= first_line {
                1
            } else {
                last_line + 1 - first_line
            }
        };

        let (start_skip, end_skip) = {
            if num_lines > 4 {
                (4, num_lines - 4)
            } else {
                (1, 0)
            }
        };

        let mut ellipses_output = false;
        for i in 0..num_lines {
            let l = first_line + i;
            if i >= start_skip && i <= end_skip {
                if !ellipses_output {
                    writeln!(f, "    |...")?;
                    ellipses_output = true;
                }
                continue;
            }
            if l >= start.line() && l <= end.line() {
                write!(f, "{:4}|  ", l)?;
            } else {
                write!(f, "    |  ")?;
            }
            self.fmt_line(f, l)?;
            writeln!(f)?;
        }
        Ok(())
    }

    /// Format text with highlighting between start and end
    ///
    /// This is the main method used by clients of the trait
    fn fmt_context(&self, fmt: &mut dyn std::fmt::Write, start: &P, end: &P) -> std::fmt::Result
    where
        P: UserPosn,
    {
        if start.line() == end.line() || (start.line() + 1 == end.line() && end.column() == 0) {
            let num_cols = {
                if start.line() == end.line() {
                    end.column() - start.column()
                } else {
                    self.line_length(start.line())
                }
            };
            self.fmt_context_single_line(fmt, start, num_cols)
        } else {
            self.fmt_context_multiple_lines(fmt, start, end)
        }
    }
}
