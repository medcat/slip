use super::*;
use lazy_static::lazy_static;
use serde_derive::*;
use std::borrow::Cow;
use std::cmp::{Eq, PartialEq};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    parts: Vec<Token>,
    generics: Option<Roll<Type>>,
    area: Span,
}

lazy_static! {
    static ref VOID_TYPE: Type = Type::from(Some("void"));
}

impl Type {
    pub fn new(parts: Vec<Token>, generics: Option<Roll<Type>>, area: Span) -> Type {
        Type {
            parts,
            generics,
            area,
        }
    }

    pub fn void() -> &'static Type {
        &VOID_TYPE
    }

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
    pub fn span_mut(&mut self) -> &mut Span {
        &mut self.area
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
            && (self.generics.is_none() || self.generics.as_ref().unwrap().len() == 0)
    }

    pub fn join(&self, other: &Self) -> Self {
        Type::join_all([self, other].iter().cloned())
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

    pub fn without_generics(&self) -> Self {
        Type {
            parts: self.parts.clone(),
            generics: None,
            area: self.area,
        }
    }
}

impl Node for Type {
    fn parse(stream: &mut TokenStream) -> Result<Type, Error> {
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

        if let Some(v) = generics.as_ref() {
            span |= v.span();
        }

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

impl<'s, T: Into<Cow<'s, str>>, I: IntoIterator<Item = T>> From<I> for Type {
    fn from(array: I) -> Type {
        let new = array
            .into_iter()
            .map(|t| Token::new(TokenKind::ModuleName, Span::default(), Some(t)))
            .collect();
        Type::new(new, None, Span::default())
    }
}

impl Eq for Type {}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let mut p = self.parts().iter().peekable();
        while let Some(n) = p.next() {
            write!(fmt, "{}", n.value().unwrap())?;
            if p.peek().is_some() {
                write!(fmt, "::")?;
            }
        }

        match &self.generics {
            Some(generics) => {
                write!(fmt, "<")?;
                let mut gen = generics.iter().peekable();
                while let Some(generic) = gen.next() {
                    write!(fmt, "{}", generic)?;
                    if gen.peek().is_some() {
                        write!(fmt, ", ")?;
                    }
                }

                write!(fmt, ">")?;
            }

            None => {}
        }

        Ok(())
    }
}
