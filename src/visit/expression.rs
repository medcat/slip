use crate::syn::function::expression as expr;
use crate::syn::function::expression::Expression;

pub trait ExpressionVisitor {
    type Value;
    type Error;

    fn parse_expression(&mut self, expr: &expr::Expression) -> Result<Self::Value, Self::Error> {
        match expr {
            Expression::Access(access) => self.parse_expression_access(access),
            Expression::Call(call) => self.parse_expression_call(call),
            Expression::Index(index) => self.parse_expression_index(index),
            Expression::Infix(infix) => self.parse_expression_infix(infix),
            Expression::Prefix(prefix) => self.parse_expression_prefix(prefix),
            Expression::Suffix(suffix) => self.parse_expression_suffix(suffix),
            Expression::Atom(atom) => self.parse_expression_atom(atom),
        }
    }

    fn parse_expression_access(
        &mut self,
        access: &expr::Access,
    ) -> Result<Self::Value, Self::Error>;
    fn parse_expression_call(&mut self, access: &expr::Call) -> Result<Self::Value, Self::Error>;
    fn parse_expression_index(&mut self, access: &expr::Index) -> Result<Self::Value, Self::Error>;
    fn parse_expression_infix(
        &mut self,
        access: &expr::InfixOperation,
    ) -> Result<Self::Value, Self::Error>;
    fn parse_expression_prefix(
        &mut self,
        access: &expr::PrefixOperation,
    ) -> Result<Self::Value, Self::Error>;
    fn parse_expression_suffix(
        &mut self,
        access: &expr::SuffixOperation,
    ) -> Result<Self::Value, Self::Error>;
    fn parse_expression_atom(&mut self, access: &expr::Atom) -> Result<Self::Value, Self::Error>;
}
