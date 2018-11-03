use super::{BasicNode, Node, Roll};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Use(Vec<Token>, Roll<UseTrail>, Span);

fn prefix<F, T>(stream: &mut TokenStream, mut act: F) -> Result<(Vec<Token>, Option<T>, Span)>
where
    F: FnMut(&mut TokenStream, &mut Span) -> Result<T>,
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

fn prefix_basic(stream: &mut TokenStream) -> Result<(Vec<Token>, Span)> {
    prefix(stream, |stream, _| {
        stream
            .error_from(&[TokenKind::ModuleName])
            .map(|_| unimplemented!())
    }).map(|(a, _, b)| (a, b))
}

impl Node for Use {
    fn parse(stream: &mut TokenStream) -> Result<Use> {
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

        let content = content.unwrap_or_else(|| Roll::empty());
        Ok(Use(prefix, content, span | inspan))
    }
}

impl BasicNode for Use {
    fn span(&self) -> Span {
        self.2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UseTrail {
    Static(Vec<Token>, Span),
    Rename(Vec<Token>, Vec<Token>, Span),
    Star(Span),
}

impl UseTrail {
    pub fn base(&self) -> &[Token] {
        match self {
            UseTrail::Static(v, _) => &v[..],
            UseTrail::Rename(v, _, _) => &v[..],
            UseTrail::Star(_) => &[],
        }
    }

    pub fn name(&self) -> Option<&[Token]> {
        match self {
            UseTrail::Static(v, _) => {
                let tail = v.len();
                Some(&v[(tail - 2)..(tail - 1)])
            }
            UseTrail::Rename(_, v, _) => Some(&v[..]),
            UseTrail::Star(_) => None,
        }
    }
}

impl Node for UseTrail {
    fn parse(stream: &mut TokenStream) -> Result<UseTrail> {
        match stream.peek_kind() {
            Some(TokenKind::ModuleName) => {
                let (val, mut span) = prefix_basic(stream)?;

                if stream.peek_one(TokenKind::As) {
                    span |= stream.expect_one(TokenKind::As)?.span();
                    let (alias, alspan) = prefix_basic(stream)?;
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
