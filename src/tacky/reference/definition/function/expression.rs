use super::super::Type;
use crate::diag::Span;
use crate::syn::function::FunctionName;
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    // a.b syntax, where a is the expression, and b is the FunctionName.
    Access(Box<Expression>, FunctionName, Span),
    Array(Vec<Expression>, Span),
    Underscore(Span),
    Ident(String, Span),
    SingleString(String, Span),
    DoubleString(String, Span),
    Integer(usize, Span),
    Float(f64, Span),
    Type(Type),
    Map(Vec<(Expression, Expression, Span)>, Span),
    Tuple(Vec<Expression>, Span),
    Call(Type, Vec<Expression>, Span),
    Index(Type, Vec<Expression>, Span),
    InfixOperation(Type, Box<Expression>, Box<Expression>, Span),
    PrefixOperation(Type, Box<Expression>, Span),
    SuffixOperation(Type, Box<Expression>, Span),
    // UnifiedCall(Box<Expression>, FunctionName, Roll<Expression>, Span),
    // StandardCall(String, Vec<Expression>, Span),
    // ExpressionCall(Box<Expression>, Roll<Expression>, Span),
}
