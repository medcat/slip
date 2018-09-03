use super::Statement;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementGroup(Vec<Statement>, Span);

impl StatementGroup {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &Statement> + 'a {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Statement> + 'a {
        self.0.iter_mut()
    }
}

impl Node for StatementGroup {
    fn parse(stream: &mut TokenStream) -> Result<StatementGroup> {
        let mut span = stream.expect_one(TokenKind::LeftBrace)?.span();
        let mut contents = vec![];
        while !stream.peek_one(TokenKind::RightBrace) {
            let stmt = Statement::parse(stream)?;
            span |= stmt.span();
            contents.push(stmt);
        }

        span |= stream.expect_one(TokenKind::RightBrace)?.span();
        Ok(StatementGroup(contents, span))
    }
}

impl BasicNode for StatementGroup {
    fn span(&self) -> Span {
        self.1
    }
}

impl<'a> IntoIterator for &'a StatementGroup {
    type Item = &'a Statement;
    type IntoIter = ::std::slice::Iter<'a, Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut StatementGroup {
    type Item = &'a mut Statement;
    type IntoIter = ::std::slice::IterMut<'a, Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for StatementGroup {
    type Item = Statement;
    type IntoIter = <Vec<Statement> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
