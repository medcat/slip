use super::SourceId;
use std::fmt;
use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The location in the source of a specific segment of text.  This
/// does not contain a reference to the source, merely offsets and
/// positions within it.
pub struct Span {
    start: Position,
    end: Position,
    source: Option<SourceId>,
}

/// The location in the source file of a specific segment of text,
/// annotated with the file name information.
pub struct SourceSpan<'s>(Span, Option<&'s str>);

impl Span {
    pub fn new(start: Position, end: Position, source: Option<SourceId>) -> Span {
        Span { start, end, source }
    }

    pub fn of_length(size: usize) -> Span {
        Span::new(Position::default(), Position::new(0, 1, size + 1), None)
    }

    pub fn identity() -> Span {
        Span::new(Position::invalid(), Position::invalid(), None)
    }

    pub fn start(&self) -> Position {
        self.start
    }
    pub fn end(&self) -> Position {
        self.end
    }

    pub fn source(&self) -> Option<SourceId> {
        self.source
    }

    // pub fn annotate<'s>(&self, set: &'s super::DiagnosticSet) -> SourceSpan<'s> {
    //     SourceSpan(
    //         *self,
    //         self.2
    //             .as_ref()
    //             .and_then(|s| set.sources.get(s))
    //             .map(|s| &s.name[..]),
    //     )
    // }

    pub fn update(&mut self, other: &Self) {
        let start = self.start.lower_or(&other.start);
        let end = self.end.upper_or(&other.end);

        self.start = start;
        self.end = end;
    }
}

impl Default for Span {
    fn default() -> Span {
        Span::of_length(0)
    }
}

impl BitOr for Span {
    type Output = Span;

    fn bitor(self, other: Self) -> Span {
        let start = self.start.lower_or(&other.start);
        let end = self.end.upper_or(&other.end);
        let source = other.source.or(self.source);
        Span { start, end, source }
    }
}

impl BitOrAssign for Span {
    fn bitor_assign(&mut self, other: Self) {
        self.update(&other)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}..{}:{}",
            self.start.line(),
            self.start.column(),
            self.end.line(),
            self.end.column()
        )
    }
}

impl<'s> fmt::Display for SourceSpan<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}..{}:{} in file {}",
            self.0.start().line(),
            self.0.start().column(),
            self.0.end().line(),
            self.0.end().column(),
            self.1.unwrap_or("(missing)")
        )
    }
}

// impl<'a> fmt::Display for Span {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // let scope = &self.1[self.0.start().offset..self.0.end().offset];
//         let count = {
//             let usize_bits = usize::min_value().count_zeros();
//             let count = usize_bits - self.1.lines().count().leading_zeros() - 1;
//             // usize_bits <= 64, and u8 has range of 0..255.
//             let count = count as u8;
//             let count: f64 = count.into();
//             (count / 10.0f64.log2()).trunc() as usize + 1
//         };
//         let start_line = self.0.start().line;
//         let end_line = self.0.end().line();
//         let lines = self
//             .1
//             .lines()
//             .enumerate()
//             .skip(start_line.saturating_sub(1))
//             .take(end_line - start_line + 1);
//         writeln!(f, "{:1$}| ", "", count + 1)?;
//         for (num, line) in lines {
//             let lino = num + 1;
//             if lino >= start_line && lino <= end_line {
//                 write!(f, "{} | ", lino)?;
//             } else {
//                 write!(f, "{:1$}| ", "", count + 1)?;
//             }

//             writeln!(f, "{}", line)?;

//             if lino == end_line {
//                 let scolumn = self.0.start().column();
//                 let ecolumn = self.0.end().column();
//                 let ncolumns = if ecolumn <= scolumn {
//                     1
//                 } else {
//                     ecolumn - scolumn
//                 };
//                 writeln!(
//                     f,
//                     "{:lineoff$}|{:scol$}{:^<repeat$}",
//                     "",
//                     "",
//                     "",
//                     repeat = ncolumns,
//                     lineoff = count + 1,
//                     scol = scolumn,
//                 )?;
//             }
//         }

//         Ok(())
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A position within the source.  This contains the offset within the
/// source, as well as line and column information.
pub struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(offset: usize, line: usize, column: usize) -> Position {
        Position {
            offset,
            line,
            column,
        }
    }

    pub fn invalid() -> Position {
        Position::new(0, 0, 0)
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn valid(&self) -> bool {
        !(self.line == 0 && self.column == 0)
    }

    fn lower_or(&self, other: &Self) -> Position {
        if !self.valid() {
            return *other;
        }
        let offset = if self.offset < other.offset {
            self.offset
        } else {
            other.offset
        };
        let line = if self.line < other.line {
            self.line
        } else {
            other.line
        };
        let column = if self.column < other.column {
            self.column
        } else {
            other.column
        };
        Position::new(offset, line, column)
    }

    fn upper_or(&self, other: &Self) -> Position {
        if !self.valid() {
            return *other;
        }
        let offset = if self.offset > other.offset {
            self.offset
        } else {
            other.offset
        };
        let line = if self.line > other.line {
            self.line
        } else {
            other.line
        };
        let column = if self.column > other.column {
            self.column
        } else {
            other.column
        };
        Position::new(offset, line, column)
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

// impl Ord for Position {
//     fn cmp(&self, other: &Self) -> Ordering {
//         println!(
//             "ord({:?}, {:?}) = {:x?}",
//             self,
//             other,
//             self.offset.cmp(&other.offset)
//         );
//         self.offset.cmp(&other.offset)
//     }
// }

// impl PartialOrd for Position {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
