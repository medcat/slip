use super::catch::Catch;
use super::group::StatementGroup;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Try {
    base: StatementGroup,
    catch: Roll<Catch>,
    // There is meaning in not having a last clause, as opposed to
    // an empty clause.
    last: Option<StatementGroup>,
    area: Span,
}

impl Node for Try {
    fn parse(stream: &mut TokenStream) -> Result<Try> {
        let mut span = stream.expect_one(TokenKind::Try)?.span();
        let base = StatementGroup::parse(stream)?;
        span |= base.span();
        let catch = Roll::<Catch>::roll(stream, None, TokenKind::Catch, None, true, false)?;

        let last = if stream.peek_one(TokenKind::Finally) {
            span |= stream.expect_one(TokenKind::Finally)?.span();
            let group = StatementGroup::parse(stream)?;
            span |= group.span();
            Some(group)
        } else {
            None
        };

        Ok(Try {
            base,
            catch,
            last,
            area: span,
        })
    }
}

impl BasicNode for Try {
    fn span(&self) -> Span {
        self.area
    }
}
