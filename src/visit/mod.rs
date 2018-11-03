mod expression;
mod root;
mod statement;

pub use self::expression::ExpressionVisitor;
pub use self::root::RootVisitor;
pub use self::statement::StatementVisitor;

type Tail<V, E> = Result<Option<V>, E>;

pub trait Visitor<'s>: RootVisitor<'s> {}
