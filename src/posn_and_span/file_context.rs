use crate::PosnInStream;
pub trait FmtContext<P> {
    fn line_length(&self, line:usize) -> usize;

    fn fmt_line(&self, f: &mut dyn std::fmt::Write, line: usize) -> std::fmt::Result;

    fn fmt_context_single_line(
        &self,
        f: &mut dyn std::fmt::Write,
        start: &P,
        num_cols: usize, // number of columns to highlight
    ) -> std::fmt::Result
    where
        P: PosnInStream,
    {
        let line = start.line();
        let first_col = start.column();
        if line > 1 {
            write!(f, "    |  ")?;
            self.fmt_line(f, line - 1)?;
            write!(f, "\n")?;
        }
        write!(f, "{:4}|  ", line)?;
        self.fmt_line(f, line)?;
        write!(f, "\n")?;
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
        write!(f, "\n")?;
        write!(f, "    |  ")?;
        write!(f, "\n")
    }

    fn fmt_context_multiple_lines(
        &self,
        f: &mut dyn std::fmt::Write,
        start: &P,
        end: &P,
    ) -> std::fmt::Result
    where
        P: PosnInStream,
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
                    write!(f, "    |...\n")?;
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
            write!(f, "\n")?;
        }
        Ok(())
    }

    fn fmt_context(&self, fmt: &mut dyn std::fmt::Write, start: &P, end: &P) -> std::fmt::Result
    where
        P: PosnInStream,
    {
        if start.line() == end.line() || (start.line() + 1 == end.line() && end.column() == 0) {
            let mut num_cols = {
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
