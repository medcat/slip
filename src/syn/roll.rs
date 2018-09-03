use super::*;

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
    ) -> Result<Roll<T>> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), true, true)
    }

    pub fn with_terminate_once(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), true, false)
    }

    pub fn with_terminate_trail(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), false, true)
    }

    pub fn with_terminate(
        stream: &mut TokenStream,
        start: TokenKind,
        sep: TokenKind,
        end: TokenKind,
    ) -> Result<Roll<T>> {
        Roll::<T>::roll(stream, Some(start), sep, Some(end), false, false)
    }

    pub fn roll(
        stream: &mut TokenStream,
        start: Option<TokenKind>,
        sep: TokenKind,
        end: Option<TokenKind>,
        at_least: bool,
        trail: bool,
    ) -> Result<Roll<T>> {
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

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut T> + 'a {
        self.0.iter_mut()
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
