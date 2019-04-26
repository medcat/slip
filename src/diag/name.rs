use super::Level;

macro_rules! diag_variant {
    (pub enum $name:tt {
        $($variant:tt = ($short:expr, $level:expr)),*
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

            pub fn short(&self) -> &'static str {
                match self {
                    $($name::$variant => $short),*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $short),)*
                }
            }
        }
    };
}

diag_variant! {
    pub enum Name {
        TestDebug = ("test-debug", Level::Debug),
        TestInfo = ("test-info", Level::Info),
        TestWarning = ("test-warning", Level::Warning),
        TestError = ("test-error", Level::Error),
        TestPanic = ("test-panic", Level::Panic),

        Note = ("note", Level::Info),

        UnexpectedToken = ("unexpected-token", Level::Panic),

        LiteralError = ("literal-error", Level::Error),
        UndefinedLocal = ("undefined-local", Level::Error),

        AmbiguousType = ("ambiguous-type", Level::Error),
        UnknownType = ("unknown-type", Level::Error),
        TypeTrace = ("type-trace", Level::Never),
        PossibleType = ("type-trace.possible", Level::Never),
        AcceptedType = ("type-trace.accepted", Level::Never),

        TypeReference = ("type-reference", Level::Info),
        NonConstExpr = ("non-const-expr", Level::Error),

        Generics = ("generics", Level::Error),
        TypeRedefinition = ("type-redefinition", Level::Error),
        FuncRedefinition = ("func-redefinition", Level::Error)


    }
}

// All,
// Debug,
// Info,
// Warning,
// Error,
// Panic,
// Off,
