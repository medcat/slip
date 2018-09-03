mod expression;
mod root;
mod statement;

pub use self::expression::ExpressionVisitor;
pub use self::root::RootVisitor;
pub use self::statement::StatementVisitor;

pub trait Visitor: RootVisitor {}
