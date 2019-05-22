use super::{Level, Name, Source, Span};
use std::borrow::Cow;
use std::io::{Result as IoResult, Write};
use term::Terminal;

#[derive(Debug, Clone)]
/// A single emission caused by a diagnostic.  This is the materialization
/// of a diagnostic.  This can - and should - always be constructed
/// regardless of whether or not the emission should be outputted to the
/// terminal, as we keep track of them.
pub(super) struct Emission {
    /// The name of the mission.  This can be used to determine the default
    /// level of the emission.
    name: Name,
    /// The level of the emission when it was emitted.
    level: Level,
    /// The area in which it was emitted.
    span: Span,
    /// The message given for emission.  This can contain debug information.
    message: Cow<'static, str>,
}

impl Emission {
    /// Constructs a new emission from the given information.  The message
    /// here can be a static string slice, or an owned string, as both are
    /// acceptable.
    pub fn new<T: Into<Cow<'static, str>>>(
        name: Name,
        level: Level,
        span: Span,
        message: T,
    ) -> Emission {
        Emission {
            name,
            level,
            span,
            message: message.into(),
        }
    }

    /// This emits out to a terminal, with the given file, if it exists.  This
    /// provides the feedback to the user, if requested.  This should not
    /// error unless there is an underlying issue with the IO object.
    pub(super) fn emit<T, TO>(&self, source: Option<&Source>, mut out: &mut T) -> IoResult<()>
    where
        TO: Write,
        T: Terminal<Output = TO> + Send + ?Sized,
    {
        use term::color;

        let start_line = self.span.start().line();
        let end_line = self.span.end().line();
        // This computes the number of base 10 characters required to fully
        // represent the end_line; and since end_line > start_line, it will
        // always be enough to represent the start line, too.
        let count = f64::from(end_line as u32).log10().trunc() as usize + 1;
        // We want to get the lines around the area that had the error.  So...
        let lines = source
            .as_ref()
            // We'll try to get the content of the file, if it exists.
            .and_then(|f| f.content())
            // If it doesn't, just use an empty string.
            .unwrap_or("")
            // Enumerate the lines,
            .lines()
            .enumerate()
            // Skipping until the start_line - 4,
            .skip(start_line.saturating_sub(4))
            // taking everything between the start and end lines, plus some
            // extra, for context.
            .take(end_line - start_line + 8);

        out.fg(color::BRIGHT_BLUE).unwrap();

        // Write the first line.  It'll write up to the count number of spaces,
        // giving enough room to make it line up later.
        writeln!(&mut out, "{:1$}> ", "", count + 1)?;

        out.reset().unwrap();

        for (num, line) in lines {
            let lino = num + 1;

            out.fg(color::BRIGHT_BLUE).unwrap();

            if lino >= start_line && lino <= end_line {
                // we're definitely in between the start and end here.  So we
                // actually want to output the line numbers.
                write!(&mut out, "{:1$} | ", lino, count + 1)?;
            } else {
                // If we're not in range, just output empty space.
                write!(&mut out, "{:1$}| ", "", count + 1)?;
            }

            out.reset().unwrap();

            // Now, actually write the line from the file.  No colouring or
            // anything.  We don't need it.
            writeln!(&mut out, "{}", line)?;
        }
        let scolumn = self.span.start().column();
        let ecolumn = self.span.end().column();
        let ncolumns = if ecolumn <= scolumn {
            1
        } else {
            ecolumn - scolumn
        };

        out.fg(color::BRIGHT_YELLOW).unwrap();
        // Mark out the columns.  Yay!
        writeln!(
            &mut out,
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

        out.fg(color).unwrap();

        writeln!(&mut out, "{}: {}", self.level, self.message)?;

        out.reset().unwrap();
        out.flush().unwrap();

        Ok(())
    }
}
