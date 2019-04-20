use super::*;
use serde_derive::*;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll<T: Node>(Vec<T>, Span);

impl<T: Node> Roll<T> {
    pub fn empty() -> Roll<T> {
        Roll(vec![], Span::identity())
    }

    pub fn with_terminate_trail_once(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>, Error> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), true, true)
    }

    pub fn with_terminate_once(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>, Error> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), true, false)
    }

    pub fn with_terminate_trail(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>, Error> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), false, true)
    }

    pub fn with_terminate(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>, Error> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), false, false)
    }

    pub fn roll(
        stream: &mut TokenStream,
        start: Option<TokenKind>,
        sep: TokenKind,
        end: Option<TokenKind>,
        at_least: bool,
        trail: bool,
    ) -> Result<Roll<T>, Error> {
        let mut span = match start {
            Some(start) => stream.expect_one(start)?.span(),
            None => Span::identity(),
        };
        let mut contents = vec![];
        let terminating = |st: &mut TokenStream| match end {
            Some(v) => st.peek_one(v),
            _ => !st.peek_one(sep),
        };
        if at_least || !terminating(stream) {
            let result = T::parse(stream)?;
            span |= result.span();
            contents.push(result);
        } else {
            if let Some(v) = end {
                span |= stream.expect_one(v)?.span();
            }
            return Ok(Roll(contents, span));
        }

        while !terminating(stream) {
            span |= stream.expect_one(sep)?.span();
            if trail && end.map(|v| stream.peek_one(v)).unwrap_or(false) {
                break;
            } else {
                let result = T::parse(stream)?;
                span |= result.span();
                contents.push(result);
            }
        }

        if let Some(v) = end {
            span |= stream.expect_one(v)?.span();
        }

        Ok(Roll(contents, span))
    }
}

impl<T: Node> Roll<T> {
    pub fn value(&self) -> &[T] {
        &self.0[..]
    }

    pub fn span(&self) -> Span {
        self.1
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut T> + 'a {
        self.0.iter_mut()
    }
}

impl<T: Node + Clone> Roll<T> {
    pub fn join(&self, other: &Self) -> Self {
        let mut value = self.value().to_vec();
        value.extend_from_slice(other.value());
        let span = value
            .iter()
            .fold(Span::identity(), |acc, val| acc | val.span());
        Roll(value, span)
    }
}

impl<'a, T: Node> IntoIterator for &'a Roll<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T: Node> IntoIterator for &'a mut Roll<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T: Node> IntoIterator for Roll<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Node + PartialEq> PartialEq for Roll<T> {
    fn eq(&self, other: &Roll<T>) -> bool {
        self.0 == other.0
    }
}

impl<T: Node + Eq> Eq for Roll<T> {}

impl<T: Node + Hash> Hash for Roll<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
