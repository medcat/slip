use super::Level;

macro_rules! diag_variant {
    (pub enum $name:tt {
        $($variant:tt = ($s:expr, $level:expr)),*
    }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            pub fn level(&self) -> $crate::diag::Level {
                match self {
                    $($name::$variant => $level),*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $s),)*
                }
            }
        }
    };
}

diag_variant! {
    pub enum Diagnostic {
        TestDebug = ("test-debug", Level::Debug),
        TestInfo = ("test-info", Level::Info),
        TestWarning = ("test-warning", Level::Warning),
        TestError = ("test-error", Level::Error),
        TestPanic = ("test-panic", Level::Panic),

        LiteralError = ("literal-error", Level::Error),
        UndefinedLocal = ("undefined-local", Level::Error)
    }
}

// All,
// Debug,
// Info,
// Warning,
// Error,
// Panic,
// Off,
