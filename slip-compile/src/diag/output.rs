use std::io::{Result as IoResult, Write};
use std::ops::{Deref, DerefMut};
use term::{color, Attr, Result, Terminal};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
/// This allows us to wrap a non-terminal target with stubbed out terminal
/// behavior, ignoring all terminal requests.  This is only really good if
/// we're using the terminal to make output pretty, not as an actual
/// interactive session.  Be careful with this.
pub(super) struct NonTerminal<T: Write>(T);

impl<T: Write> NonTerminal<T> {
    pub(super) fn new(inner: T) -> NonTerminal<T> {
        NonTerminal(inner)
    }
    pub(super) fn into_inner(self) -> T {
        self.0
    }
}

impl NonTerminal<::std::io::Stderr> {
    pub(super) fn stderr() -> NonTerminal<::std::io::Stderr> {
        NonTerminal(::std::io::stderr())
    }
}

impl<T: Write> Write for NonTerminal<T> {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> IoResult<()> {
        self.0.flush()
    }
}

impl<T: Write> Terminal for NonTerminal<T> {
    type Output = T;
    fn fg(&mut self, _color: color::Color) -> Result<()> {
        Ok(())
    }
    fn bg(&mut self, _color: color::Color) -> Result<()> {
        Ok(())
    }
    fn attr(&mut self, _attr: Attr) -> Result<()> {
        Ok(())
    }
    fn supports_attr(&self, _attr: Attr) -> bool {
        false
    }
    fn reset(&mut self) -> Result<()> {
        Ok(())
    }
    fn supports_reset(&self) -> bool {
        true
    }
    fn supports_color(&self) -> bool {
        true
    }
    fn cursor_up(&mut self) -> Result<()> {
        Ok(())
    }
    fn delete_line(&mut self) -> Result<()> {
        Ok(())
    }
    fn carriage_return(&mut self) -> Result<()> {
        Ok(())
    }
    fn get_ref(&self) -> &Self::Output {
        &self.0
    }
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
    fn into_inner(self) -> Self::Output
    where
        Self: Sized,
    {
        self.0
    }
}

impl<T: Write> Deref for NonTerminal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Write> DerefMut for NonTerminal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
