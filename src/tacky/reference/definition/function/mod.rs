use crate::diag::Span;
use serde_derive::*;

mod expression;
mod statement;

pub use self::expression::Expression;
pub use self::statement::Statement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function(Vec<Statement>, Span);
