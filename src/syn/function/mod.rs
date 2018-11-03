use self::statement::StatementGroup;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Roll, Type};
use serde_derive::*;

pub mod expression;
pub mod statement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    name: FunctionName,
    generics: Roll<Type>,
    parameters: Roll<FunctionParameter>,
    retval: Option<Type>,
    body: Option<StatementGroup>,
    area: Span,
}

impl Function {
    pub fn name(&self) -> &FunctionName {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut FunctionName {
        &mut self.name
    }

    pub fn generics(&self) -> &Roll<Type> {
        &self.generics
    }

    pub fn generics_mut(&mut self) -> &mut Roll<Type> {
        &mut self.generics
    }

    pub fn parameters(&self) -> &Roll<FunctionParameter> {
        &self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut Roll<FunctionParameter> {
        &mut self.parameters
    }

    pub fn retval(&self) -> &Option<Type> {
        &self.retval
    }

    pub fn retval_mut(&mut self) -> &mut Option<Type> {
        &mut self.retval
    }

    pub fn body(&self) -> &Option<StatementGroup> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Option<StatementGroup> {
        &mut self.body
    }
}

impl Node for Function {
    fn parse(stream: &mut TokenStream) -> Result<Function> {
        let mut span = stream.expect_one(TokenKind::Fn)?.span();
        let name = FunctionName::parse(stream)?;
        span |= name.span();
        let generics = if stream.peek_one(TokenKind::LessThan) {
            let roll = Roll::with_terminate_once(
                stream,
                TokenKind::LessThan,
                TokenKind::Comma,
                TokenKind::GreaterThan,
            )?;
            span |= roll.span();
            roll
        } else {
            Roll::empty()
        };
        let parameters = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftParen,
            TokenKind::Comma,
            TokenKind::RightParen,
        )?;
        span |= parameters.span();
        let retval = if stream.peek_one(TokenKind::Colon) {
            span |= stream.expect_one(TokenKind::Colon)?.span();
            let kind = Type::parse(stream)?;
            span |= kind.span();
            Some(kind)
        } else {
            None
        };
        let body = if stream.peek_one(TokenKind::LeftBrace) {
            let body = StatementGroup::parse(stream)?;
            span |= body.span();
            Some(body)
        } else {
            None
        };

        Ok(Function {
            name,
            generics,
            parameters,
            retval,
            body,
            area: span,
        })
    }
}

impl BasicNode for Function {
    fn span(&self) -> Span {
        self.area
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionName {
    Identifier(Token),
    Operator(Token),
    Brackets(Token, Token, Span),
    Integer(Token),
}

static FUNCTION_NAME_OPERATORS: &[TokenKind] = &[
    TokenKind::Star,
    TokenKind::Modulo,
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Divide,
    TokenKind::LessThan,
    TokenKind::GreaterThan,
    TokenKind::LessThanEqual,
    TokenKind::GreaterThanEqual,
    TokenKind::BitwiseAnd,
    TokenKind::BitwiseOr,
    TokenKind::BitwiseXor,
    TokenKind::BitwiseNot,
    TokenKind::LogicalOr,
    TokenKind::LogicalNot,
    TokenKind::LogicalAnd,
    TokenKind::RightShift,
    TokenKind::LeftShift,
    TokenKind::Compare,
    TokenKind::Spaceship,
];

impl Node for FunctionName {
    fn parse(stream: &mut TokenStream) -> Result<FunctionName> {
        match stream.peek_kind() {
            Some(TokenKind::Identifier) => Ok(FunctionName::Identifier(
                stream.expect_one(TokenKind::Identifier)?,
            )),
            Some(TokenKind::Integer) => Ok(FunctionName::Integer(
                stream.expect_one(TokenKind::Integer)?,
            )),
            Some(TokenKind::LeftBracket) => {
                let left = stream.expect_one(TokenKind::LeftBracket)?;
                let right = stream.expect_one(TokenKind::RightBracket)?;
                let span = left.span() | right.span();
                Ok(FunctionName::Brackets(left, right, span))
            }
            Some(v) if FUNCTION_NAME_OPERATORS.contains(&v) => Ok(FunctionName::Operator(
                stream.expect_any(FUNCTION_NAME_OPERATORS)?,
            )),
            _ => {
                let mut expected = vec![TokenKind::Identifier, TokenKind::LeftBracket];
                expected.extend(FUNCTION_NAME_OPERATORS);
                stream.error_from(&expected[..]).map(|_| unreachable!())
            }
        }
    }
}

impl BasicNode for FunctionName {
    fn span(&self) -> Span {
        match self {
            FunctionName::Identifier(ident) => ident.span(),
            FunctionName::Integer(int) => int.span(),
            FunctionName::Operator(op) => op.span(),
            FunctionName::Brackets(_, _, span) => *span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionParameter {
    Static(Token, Type),
    This(Token),
    Ignore(Token, Type),
}

impl Node for FunctionParameter {
    fn parse(stream: &mut TokenStream) -> Result<FunctionParameter> {
        match stream.peek_kind() {
            Some(TokenKind::This) => {
                Ok(FunctionParameter::This(stream.expect_one(TokenKind::This)?))
            }
            Some(TokenKind::Underscore) => {
                let under = stream.expect_one(TokenKind::Underscore)?;
                stream.expect_one(TokenKind::Colon)?;
                let kind = Type::parse(stream)?;
                Ok(FunctionParameter::Ignore(under, kind))
            }
            Some(TokenKind::Identifier) => {
                let under = stream.expect_one(TokenKind::Identifier)?;
                stream.expect_one(TokenKind::Colon)?;
                let kind = Type::parse(stream)?;
                Ok(FunctionParameter::Static(under, kind))
            }
            _ => stream
                .error_from(&[
                    TokenKind::This,
                    TokenKind::Underscore,
                    TokenKind::Identifier,
                ]).map(|_| unreachable!()),
        }
    }
}

impl BasicNode for FunctionParameter {
    fn span(&self) -> Span {
        match self {
            FunctionParameter::Static(tok, kind) => tok.span() | kind.span(),
            FunctionParameter::This(tok) => tok.span(),
            FunctionParameter::Ignore(tok, kind) => tok.span() | kind.span(),
        }
    }
}
