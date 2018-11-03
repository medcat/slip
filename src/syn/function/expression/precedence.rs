use crate::stream::TokenKind;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// This defines the precedence for expression operations.  Essentially,
/// when we see an infix operator, we have to choose between consuming
/// it ("staying"), or return up the stack ("breaking").  This defines
/// an ordered list, from the greatest precedence (lowest value), to
/// the lowest precedence (greatest value).  It also includes
/// supplementary data - associtivity.  In case the precedence of the
/// expression we hold is equal to the precedence of the token we see,
/// we have to choose to stay or break.  Left-associative stays, whereas
/// Right-associative breaks.
pub(super) enum Precedence {
    /// Suffix operations, as well as function calls, array subscripting,
    /// or member accesses.  This is left-associative.
    SuffixPlusCallAccess,
    /// Prefix operations, such as increment and decrement, as well
    /// as unary plus and minus, and logical and bitwise negations.
    /// This is right-associative.
    PrefixPlusLogical,
    /// Multiplication, division, and remainder operations (`*`, `/`,
    /// `%`).  Left-associative.
    StarDivide,
    /// Addition and subtraction.  Left-associative.
    PlusMinus,
    /// Bitwise shifts (`>>`, `<<`).  Left-associative.
    Shift,
    /// Ordering operations (`>`, `>=`, `<`, `<=`).  Left-associative.
    Ord,
    /// Equality operations (`==`, `!=`). Left-associative.
    Eq,
    /// Bitwise and (`&`).  Left-associative.
    BitwiseAnd,
    /// Bitwise xor (`^`).  Left-associative.
    BitwiseXor,
    /// Bitwise or (`|`).  Left-associative.
    BitwiseOr,
    /// Logical and (`&&`).  Left-associative.
    LogicalAnd,
    /// Logical or (`||`).  Left-associative.
    LogicalOr,
    /// All assignment operations, e.g. `=`, `*=`, `+=`, `-=`, ...
    /// Right-associative.
    Assign,
    /// The "default" precedence; e.g. when in a block.  This means that
    /// there is no outer expression to keep.  This is left-associative,
    /// where applicable, such that if a choice must be made between
    /// parsing a new Default-precedence token, or breaking out to a
    /// Default-precedence expression, it will stay inside.
    Default,
    /// The none precedence; e.g. there is no applicable token left.
    /// This means that the next token is not a token that can be
    /// infixed between two expressions.  This is right-associative,
    /// where applicable, such that if a choice must be made between
    /// parsing a new None-precedence token, or breaking out to a
    /// None-precedence expression, it will break out.
    None,
}

impl Precedence {
    pub(super) fn stay(self, other: Self) -> bool {
        if self == other {
            match self {
                Precedence::PrefixPlusLogical | Precedence::Assign | Precedence::None => false,
                _ => true,
            }
        } else {
            self > other
        }
    }
}

impl Into<Precedence> for TokenKind {
    fn into(self) -> Precedence {
        match self {
            TokenKind::LeftParen
            | TokenKind::Period
            | TokenKind::DoublePlus
            | TokenKind::DoubleMinus => Precedence::SuffixPlusCallAccess,
            TokenKind::Star | TokenKind::Divide | TokenKind::Modulo => Precedence::StarDivide,
            TokenKind::Plus | TokenKind::Minus => Precedence::PlusMinus,
            TokenKind::LeftShift | TokenKind::RightShift => Precedence::Shift,
            TokenKind::LessThan
            | TokenKind::LessThanEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanEqual => Precedence::Ord,
            TokenKind::Compare | TokenKind::NotEqual => Precedence::Eq,
            TokenKind::BitwiseAnd => Precedence::BitwiseAnd,
            TokenKind::BitwiseXor => Precedence::BitwiseXor,
            TokenKind::BitwiseOr => Precedence::BitwiseOr,
            TokenKind::LogicalAnd => Precedence::LogicalAnd,
            TokenKind::LogicalOr => Precedence::LogicalOr,
            TokenKind::Equals => Precedence::Assign,
            _ => Precedence::None,
        }
    }
}

impl Into<Precedence> for Option<TokenKind> {
    fn into(self) -> Precedence {
        self.map(|v| v.into()).unwrap_or(Precedence::None)
    }
}
