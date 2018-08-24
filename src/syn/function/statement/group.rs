use super::Statement;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementGroup(Vec<Statement>, Span);

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
