use super::super::diag::*;
use regex::{Regex, RegexSet, RegexSetBuilder};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A token
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
    pub(crate) value: Option<String>,
}

impl Token {
    /// Creates a new token with the given kind, span, and value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// let token = Token::new(TokenKind::Return, Span::of_length(6), Some("return"));
    /// // here, token is of type Token<'static>, because the str slice
    /// // passed has lifetime 'static.
    /// # }
    /// ```
    pub fn new<'s, T>(kind: TokenKind, span: Span, value: Option<T>) -> Token
    where
        T: Into<Cow<'s, str>>,
    {
        Token {
            kind,
            span,
            value: value.map(|v| v.into().into_owned()),
        }
    }

    /// The kind of the token.  This is used in parsing to denote what
    /// the segment of text means - for example, the string `"return"`
    /// is a `TokenKind::Return`, and should only appear where a return
    /// statement should occur.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// let source = "return 5;"
    /// let lexer = Lexer::new(source);
    /// let token = lexer.next().unwrap();
    /// assert_eq!(token.kind(), TokenKind::Return);
    /// # }
    /// ```
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// The "span" of text the token is over.  This span isn't
    /// completely correct in some aspects - specifically, when dealing
    /// with columns.  The span keeps track of the line numbers and
    /// column numbers of the start and end of the token, as well as
    /// the offset within the actual source.
    ///
    /// # Example
    /// ```rust
    /// # fn main() {
    /// let source = "return 5;"
    /// let lexer = Lexer::new(source);
    /// let token = lexer.next().unwrap();
    /// let start = Position::new(0, 1, 1);
    /// let end = Position::new(6, 1, 1);
    /// assert_eq!(token.span(), Span::new(start, end));
    /// # }
    /// ```
    pub fn span(&self) -> Span {
        self.span
    }

    /// The "value" of the token.  This only matters for tokens whose
    /// values matter - for example, numbers and identifiers.
    ///
    /// Note that the reference lifetime is elided here, and because of
    /// the rules of elision, the reference lifetime of the output
    /// string is tied to the reference lifetime of `self`.  This is
    /// intentional, due to the way the reference to the source is
    /// stored in the token.  Specifically, the token can have a
    /// reference to the source, or actually own a slice of the source.
    ///
    /// # Example
    /// ```rust
    /// # fn main() {
    /// let source = "return 5;"
    /// let lexer = Lexer::new(source);
    /// let token = lexer.next().unwrap();
    /// assert_eq!(token.value(), Some("return"));
    /// # }
    /// ```
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| &s[..])
    }

    /// Converts this token into an "unvalued" token.  Essentially, the
    /// value of the token is dropped, leaving no reference and no
    /// value.  This is an alternative to [`into_owned`] in cases
    /// where the value is not needed.  This allows us to change the
    /// lifetime of the token to be `'static`.
    ///
    /// # Example
    /// ```rust
    /// # fn main() {
    /// let source = "return 5;"
    /// let lexer = Lexer::new(source);
    /// let token = lexer.next().unwrap();
    /// let token = tokwn.into_unvalued();
    /// assert_eq!(token.value(), None);
    /// # }
    /// ```
    pub fn into_unvalued(self) -> Token {
        Token {
            kind: self.kind,
            span: self.span,
            value: None,
        }
    }

    pub fn take_value(&mut self) -> Option<String> {
        self.value.take()
    }
}

macro_rules! define_tokens {
    ($(#[$out:meta])* pub enum $name:ident {
        $(
            $(#[$in:meta])*
            $var:ident($val:expr, $pattern:expr, $display:expr)
        ),*
    }) => {
        $(#[$out])*
        pub enum $name {
            $(
                $(#[$in])*
                $var,
            )*
        }

        static PATTERNS: &'static [(TokenKind, &'static str)] = &[
            $(
                ($name::$var, $pattern),
            )*
        ];

        impl $name {
            pub(super) fn set() -> &'static RegexSet {
                lazy_static! {
                    static ref SET: RegexSet = {
                        let mut set = RegexSetBuilder::new(PATTERNS.iter().map(|(_, p)| p));
                        set.multi_line(true);
                        set.build().unwrap()
                    };
                }

                &SET
            }

            pub(super) fn value() -> &'static [Regex] {
                lazy_static! {
                    static ref VALUES: Vec<Regex> = PATTERNS
                        .iter()
                        .map(|(_, v)| Regex::new(v).unwrap())
                        .collect::<Vec<_>>();
                }

                &VALUES[..]
            }

            pub(super) fn tokens() -> &'static [TokenKind] {
                lazy_static! {
                    static ref TOKENS: Vec<TokenKind> =
                        PATTERNS.iter().map(|(k, _)| *k).collect::<Vec<_>>();
                }

                &TOKENS[..]
            }

            pub fn has_value(&self) -> bool {
                match self {
                    $(
                        $name::$var => $val == Some(true),
                    )*
                }
            }

            pub fn ignore(&self) -> bool {
                match self {
                    $(
                        $name::$var => $val == None,
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $(
                        $name::$var => write!(f, "{}", $display),
                    )*
                }
            }
        }
    };
}

define_tokens! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum TokenKind {
        Finally(Some(false), r"\Afinally", r#""finally""#),
        Module(Some(false), r"\Amodule", r#""module""#),
        Return(Some(false), r"\Areturn", r#""return""#),
        Struct(Some(false), r"\Astruct", r#""struct""#),
        Unless(Some(false), r"\Aunless", r#""unless""#),
        Catch(Some(false), r"\Acatch", r#""catch""#),
        Elsif(Some(false), r"\Aelsif", r#""elsif""#),
        Match(Some(false), r"\Amatch", r#""match""#),
        Trait(Some(false), r"\Atrait", r#""trait""#),
        While(Some(false), r"\Awhile", r#""while""#),
        Else(Some(false), r"\Aelse", r#""else""#),
        Enum(Some(false), r"\Aenum", r#""enum""#),
        This(Some(false), r"\Aself", r#""self""#),
        When(Some(false), r"\Awhen", r#""when""#),
        For(Some(false), r"\Afor", r#""for""#),
        End(Some(false), r"\Aend", r#""end""#),
        Let(Some(false), r"\Alet", r#""let""#),
        Try(Some(false), r"\Atry", r#""try""#),
        Use(Some(false), r"\Ause", r#""use""#),
        Spaceship(Some(false), r"\A<=>", r#""<=>""#),
        As(Some(false), r"\Aas", r#""as""#),
        Compare(Some(false), r"\A==", r#""==""#),
        DoubleColon(Some(false), r"\A::", r#""::""#),
        DoubleMinus(Some(false), r"\A--", r#""--""#),
        Do(Some(false), r"\Ado", r#""do""#),
        DoublePlus(Some(false), r"\A++", r#""++""#),
        Fn(Some(false), r"\Afn", r#""fn""#),
        If(Some(false), r"\Aif", r#""if""#),
        In(Some(false), r"\Ain", r#""in""#),
        LeftShift(Some(false), r"\A<<", r#""<<""#),
        LessThanEqual(Some(false), r"\A<=", r#""<=""#),
        LogicalAnd(Some(false), r"\A&&", r#""&&""#),
        NotEqual(Some(false), r"\A!=", r#""!=""#),
        LogicalOr(Some(false), r"\A||", r#""||""#),
        RightShift(Some(false), r"\A>>", r#"">>""#),
        GreaterThanEqual(Some(false), r"\A>=", r#"">=""#),
        Rocket(Some(false), r"\A=>", r#""=>""#),
        BitwiseAnd(Some(false), r"\A&", r#""&""#),
        BitwiseNot(Some(false), r"\A~", r#""~""#),
        BitwiseOr(Some(false), r"\A|", r#""|""#),
        BitwiseXor(Some(false), r"\A^", r#""^""#),
        Colon(Some(false), r"\A:", r#"":""#),
        Comma(Some(false), r"\A,", r#"",""#),
        Divide(Some(false), r"\A/", r#""/""#),
        Equals(Some(false), r"\A=", r#""=""#),
        LessThan(Some(false), r"\A<", r#""<""#),
        LeftBrace(Some(false), r"\A{", r#""{""#),
        LeftBracket(Some(false), r"\A[", r#""[""#),
        LogicalNot(Some(false), r"\A!", r#""!""#),
        LeftParen(Some(false), r"\A(", r#""(""#),
        Minus(Some(false), r"\A-", r#""-""#),
        Modulo(Some(false), r"\A%", r#""%""#),
        Star(Some(false), r"\A*", r#""*""#),
        Period(Some(false), r"\A.", r#"".""#),
        Plus(Some(false), r"\A+", r#""+""#),
        GreaterThan(Some(false), r"\A>", r#"">""#),
        RightBrace(Some(false), r"\A}", r#""}""#),
        RightBracket(Some(false), r"\A]", r#""]""#),
        RightParen(Some(false), r"\A)", r#"")""#),
        Semicolon(Some(false), r"\A;", r#"";""#),
        Underscore(Some(false), r"\A_", r#""_""#),
        Identifier(Some(true), r"\A(?:[a-z][a-zA-Z\d_-]*[!?]?|@[-+]{2}|[-+]{1,2}@)", "Identifier"),
        ModuleName(Some(true), r"\A[A-Z][a-zA-Z\d]*", "ModuleName"),
        Integer(Some(true), r"\A(0x[[:xdigit:]]+|\d+|0[0-8]+|0b[01]+)", "Integer"),
        Float(Some(true), r"\A\d+\.\d+([eE][+-]?\d+)?", "Float"),
        Comment(None as Option<bool>, r"\A//.+\n", "Comment"),
        DoubleString(Some(true), r#"\A"([^"]|\\")*""#, "DoubleString"),
        SingleString(Some(true), r"\A'[^']*'", "SingleString"),
        Escape(Some(true), r"\A\\[a-z][a-zA-Z\d_-]*[!?]?", "Escape"),
        Whitespace(None as Option<bool>, r"\A\s+", "Whitespace"),
        Eof(Some(false), r"\A", "Eof")
    }
}
