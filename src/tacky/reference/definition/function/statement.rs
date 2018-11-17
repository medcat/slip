use super::Expression;
use crate::diag::Span;
use crate::tacky::reference::Key;
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    For(String, Expression, Vec<Statement>, Span),
    If(Vec<(Expression, Vec<Statement>, Span)>, Span),
    Let(String, Option<Key>, Option<Expression>, Span),
    Return(Option<Expression>, Span),
    Try(
        Vec<Statement>,
        Vec<(String, Key, Vec<Statement>, Span)>,
        Option<Vec<Statement>>,
        Span,
    ),
    Unless(Expression, Vec<Statement>, Span),
    While(Expression, Vec<Statement>, Span),
    Expression(Expression, Span),
}
