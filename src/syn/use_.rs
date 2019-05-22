use super::{BasicNode, Node, Roll, Type};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Use(Type, Roll<UseTrail>, Span);

impl Use {
    pub fn prefix(&self) -> &Type {
        &self.0
    }
    pub fn trails(&self) -> &[UseTrail] {
        self.1.value()
    }
}

fn prefix<F, T>(
    stream: &mut TokenStream,
    mut act: F,
) -> Result<(Vec<Token>, Option<T>, Span), Error>
where
    F: FnMut(&mut TokenStream, &mut Span) -> Result<T, Error>,
{
    let current = stream.expect_one(TokenKind::ModuleName)?;
    let mut span = current.span();
    let mut value = None;
    let mut content = vec![current];

    while stream.peek_one(TokenKind::DoubleColon) {
        span |= stream.expect_one(TokenKind::DoubleColon)?.span();
        if !stream.peek_one(TokenKind::ModuleName) {
            value = Some(act(stream, &mut span)?);
            break;
        } else {
            let current = stream.expect_one(TokenKind::ModuleName)?;
            span |= current.span();
            content.push(current);
        }
    }

    Ok((content, value, span))
}

fn prefix_basic(stream: &mut TokenStream) -> Result<(Vec<Token>, Span), Error> {
    prefix(stream, |stream, _| {
        stream
            .error_from(&[TokenKind::ModuleName])
            .map(|_| unimplemented!())
    })
    .map(|(a, _, b)| (a, b))
}

impl Node for Use {
    fn parse(stream: &mut TokenStream) -> Result<Use, Error> {
        let span = stream.expect_one(TokenKind::Use)?.span();

        let (prefix, content, inspan) = prefix(stream, |stream, span| {
            let roll = Roll::with_terminate_trail_once(
                stream,
                TokenKind::LeftBrace,
                TokenKind::Comma,
                TokenKind::RightBrace,
            )?;
            *span |= roll.span();
            Ok(roll)
        })?;

        let prespan = prefix
            .iter()
            .fold(Span::identity(), |acc, el| acc | el.span());

        let kind = Type::new(prefix, None, prespan);
        let tok = stream.expect_one(TokenKind::Semicolon)?.span();
        let content = content.unwrap_or_else(Roll::empty);
        Ok(Use(kind, content, span | inspan | tok))
    }
}

impl BasicNode for Use {
    fn span(&self) -> Span {
        self.2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum UseTrail {
    Static(Type, Span),
    Rename(Type, Type, Span),
    Star(Span),
}

impl UseTrail {
    pub fn base(&self) -> Option<&Type> {
        match self {
            UseTrail::Static(v, _) => Some(&v),
            UseTrail::Rename(v, _, _) => Some(&v),
            UseTrail::Star(_) => None,
        }
    }

    pub fn name(&self) -> Option<&[Token]> {
        match self {
            UseTrail::Static(v, _) => Some(&v.parts()[(v.parts().len() - 2)..]),
            UseTrail::Rename(_, v, _) => Some(v.parts()),
            UseTrail::Star(_) => None,
        }
    }

    pub fn applies(&self, ty: &super::Type) -> bool {
        match self.name() {
            None => true,
            Some(part) if part == ty.parts() => true,
            _ => false,
        }
    }

    pub fn combine<'s>(&'s self, prefix: &'s Type, current: &'s Type) -> Vec<&'s Type> {
        match self {
            UseTrail::Static(_, _) | UseTrail::Star(_) => vec![prefix, current],
            UseTrail::Rename(from, _, _) => vec![prefix, from],
        }
    }
}

impl Node for UseTrail {
    fn parse(stream: &mut TokenStream) -> Result<UseTrail, Error> {
        match stream.peek_kind() {
            Some(TokenKind::ModuleName) => {
                let (val, mut span) = prefix_basic(stream)?;
                let val = Type::new(val, None, span);

                if stream.peek_one(TokenKind::As) {
                    span |= stream.expect_one(TokenKind::As)?.span();
                    let (alias, alspan) = prefix_basic(stream)?;
                    let alias = Type::new(alias, None, alspan);
                    span |= alspan;
                    Ok(UseTrail::Rename(val, alias, span))
                } else {
                    Ok(UseTrail::Static(val, span))
                }
            }
            Some(TokenKind::Star) => {
                let span = stream.expect_one(TokenKind::Star)?.span();
                Ok(UseTrail::Star(span))
            }
            _ => stream
                .error_from(&[TokenKind::ModuleName, TokenKind::Star])
                .map(|_| unreachable!()),
        }
    }
}

impl BasicNode for UseTrail {
    fn span(&self) -> Span {
        match self {
            UseTrail::Rename(_, _, span) => *span,
            UseTrail::Static(_, span) => *span,
            UseTrail::Star(span) => *span,
        }
    }
}
