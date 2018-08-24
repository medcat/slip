use super::{Diagnostic, Level, Span};
use std::io::{Result as IoResult, Write};
use term::Terminal;

#[derive(Debug, Clone)]
pub(super) struct Instance {
    which: Diagnostic,
    level: Level,
    span: Span,
    message: String,
}

impl Instance {
    pub fn new(which: Diagnostic, level: Level, span: Span, message: String) -> Instance {
        Instance {
            which,
            level,
            span,
            message,
        }
    }

    pub fn term_emit<T>(&self, source: &str, term: &mut T) -> IoResult<()>
    where
        T: Terminal + ?Sized,
    {
        use term::color;

        let start_line = self.span.start().line();
        let end_line = self.span.end().line();
        let count = {
            let usize_bits = usize::min_value().count_zeros();
            let count = usize_bits - end_line.leading_zeros() - 1;
            // usize_bits <= 64, and u8 has range of 0..255.
            let count = count as u8;
            let count: f64 = count.into();
            (count / 10.0f64.log2()).trunc() as usize + 1
        };
        let lines = source
            .lines()
            .enumerate()
            .skip(start_line.saturating_sub(4))
            .take(end_line - start_line + 4);

        term.fg(color::BRIGHT_BLUE).unwrap();
        writeln!(term, "{:1$}> ", "", count + 1)?;
        term.reset().unwrap();
        for (num, line) in lines {
            let lino = num + 1;
            term.fg(color::BRIGHT_BLUE).unwrap();
            if lino >= start_line && lino <= end_line {
                write!(term, "{} | ", lino)?;
            } else {
                write!(term, "{:1$}| ", "", count + 1)?;
            }

            term.reset().unwrap();
            writeln!(term, "{}", line)?;
        }
        let scolumn = self.span.start().column();
        let ecolumn = self.span.end().column();
        let ncolumns = if ecolumn <= scolumn {
            1
        } else {
            ecolumn - scolumn
        };
        term.fg(color::BRIGHT_YELLOW).unwrap();
        writeln!(
            term,
            "{:lineoff$}>{:scol$}{:^<repeat$}",
            "",
            "",
            "",
            repeat = ncolumns,
            lineoff = count + 1,
            scol = scolumn,
        )?;

        let color = match self.level {
            Level::Panic => color::BRIGHT_MAGENTA,
            Level::Error => color::BRIGHT_RED,
            Level::Warning => color::YELLOW,
            _ => color::CYAN,
        };

        term.fg(color).unwrap();
        writeln!(term, "{}: {}", self.level, self.message)?;
        term.reset().unwrap();
        term.flush().unwrap();

        Ok(())
    }

    pub fn file_emit<W>(&self, source: &str, write: &mut W) -> IoResult<()>
    where
        W: Write + ?Sized,
    {
        let start_line = self.span.start().line();
        let end_line = self.span.end().line();
        let count = {
            let usize_bits = usize::min_value().count_zeros();
            let count = usize_bits - end_line.leading_zeros() - 1;
            // usize_bits <= 64, and u8 has range of 0..255.
            let count = count as u8;
            let count: f64 = count.into();
            (count / 10.0f64.log2()).trunc() as usize + 1
        };
        let lines = source
            .lines()
            .enumerate()
            .skip(start_line.saturating_sub(4))
            .take(end_line - start_line + 4);

        writeln!(write, "{:1$}> ", "", count + 1)?;
        for (num, line) in lines {
            let lino = num + 1;
            if lino >= start_line && lino <= end_line {
                write!(write, "{} | ", lino)?;
            } else {
                write!(write, "{:1$}| ", "", count + 1)?;
            }

            writeln!(write, "{}", line)?;
        }
        let scolumn = self.span.start().column();
        let ecolumn = self.span.end().column();
        let ncolumns = if ecolumn <= scolumn {
            1
        } else {
            ecolumn - scolumn
        };

        writeln!(
            write,
            "{:lineoff$}>{:scol$}{:^<repeat$}",
            "",
            "",
            "",
            repeat = ncolumns,
            lineoff = count + 1,
            scol = scolumn,
        )?;

        writeln!(write, "{}: {}", self.level, self.message)?;

        Ok(())
    }
}
