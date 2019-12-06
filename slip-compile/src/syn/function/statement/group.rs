use super::Statement;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementGroup {
    contents: Vec<Statement>,
    area: Span,
}

impl StatementGroup {
    pub fn statements(&self) -> &[Statement] {
        &self.contents
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &Statement> + 'a {
        self.contents.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Statement> + 'a {
        self.contents.iter_mut()
    }
}

impl Node for StatementGroup {
    fn parse(stream: &mut TokenStream) -> Result<StatementGroup, Error> {
        let mut span = stream.expect_one(TokenKind::LeftBrace)?.span();
        let mut contents = vec![];
        while !stream.peek_one(TokenKind::RightBrace) {
            let stmt = Statement::parse(stream)?;
            span |= stmt.span();
            contents.push(stmt);
        }

        span |= stream.expect_one(TokenKind::RightBrace)?.span();
        Ok(StatementGroup {
            contents,
            area: span,
        })
    }
}

impl BasicNode for StatementGroup {
    fn span(&self) -> Span {
        self.area
    }
}

impl<'a> IntoIterator for &'a StatementGroup {
    type Item = &'a Statement;
    type IntoIter = ::std::slice::Iter<'a, Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl<'a> IntoIterator for &'a mut StatementGroup {
    type Item = &'a mut Statement;
    type IntoIter = ::std::slice::IterMut<'a, Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter_mut()
    }
}

impl<'a> IntoIterator for StatementGroup {
    type Item = Statement;
    type IntoIter = <Vec<Statement> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}
