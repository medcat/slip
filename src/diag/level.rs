use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Level {
    All,
    Debug,
    Info,
    Warning,
    Error,
    Panic,
    Off,
}

impl Default for Level {
    fn default() -> Level {
        Level::Info
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::All => write!(f, "all"),
            Level::Debug => write!(f, "debug"),
            Level::Info => write!(f, "info"),
            Level::Warning => write!(f, "warning"),
            Level::Error => write!(f, "error"),
            Level::Panic => write!(f, "panic"),
            Level::Off => write!(f, "off"),
        }
    }
}
