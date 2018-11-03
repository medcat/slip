use super::*;
use serde_derive::*;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    parts: Vec<Token>,
    generics: Option<Roll<Type>>,
    area: Span,
}

impl Type {
    pub fn parts(&self) -> &[Token] {
        &self.parts[..]
    }
    pub fn parts_mut(&mut self) -> &mut [Token] {
        &mut self.parts[..]
    }
    pub fn generics(&self) -> &Option<Roll<Type>> {
        &self.generics
    }
    pub fn generics_mut(&mut self) -> &mut Option<Roll<Type>> {
        &mut self.generics
    }

    pub fn join(&self, other: &Self) -> Self {
        Type::join_all([self, other].into_iter().map(|v| *v))
    }

    pub fn join_all<'a, T: IntoIterator<Item = &'a Type>>(it: T) -> Self {
        let mut parts = vec![];
        let mut area = Span::identity();

        let generics = it
            .into_iter()
            .fold(None as Option<Roll<Type>>, |generics, kind| {
                parts.extend_from_slice(kind.parts());
                area = kind.span();
                match generics {
                    None => kind.generics().clone(),
                    Some(current) => Some(
                        kind.generics()
                            .as_ref()
                            .map(|val| current.join(val))
                            .unwrap_or_else(|| current),
                    ),
                }
            });

        Type {
            parts,
            generics,
            area,
        }
    }
}

impl Node for Type {
    fn parse(stream: &mut TokenStream) -> Result<Type> {
        let start = stream.expect_one(TokenKind::ModuleName)?;
        let mut span = start.span();
        let mut contents = vec![start];

        while stream.peek_one(TokenKind::DoubleColon) {
            span |= stream.expect_one(TokenKind::DoubleColon)?.span();
            let result = stream.expect_one(TokenKind::ModuleName)?;
            span |= result.span();
            contents.push(result);
        }

        let generics = if stream.peek_one(TokenKind::LessThan) {
            Some(Roll::with_terminate_once(
                stream,
                TokenKind::LessThan,
                TokenKind::Comma,
                TokenKind::GreaterThan,
            )?)
        } else {
            None
        };

        generics.as_ref().map(|v| span |= v.span());

        Ok(Type {
            parts: contents,
            generics,
            area: span,
        })
    }
}

impl BasicNode for Type {
    fn span(&self) -> Span {
        self.area
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parts.hash(state);
        self.generics.hash(state);
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        self.parts == other.parts && self.generics == other.generics
    }
}

impl Eq for Type {}
