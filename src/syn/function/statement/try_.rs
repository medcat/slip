use super::catch::Catch;
use super::group::StatementGroup;
use crate::diag::Span;
use crate::error::*;
use crate::stream::{TokenKind, TokenStream};
use crate::syn::{BasicNode, Node, Roll};
use serde_derive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Try {
    base: StatementGroup,
    catch: Roll<Catch>,
    // There is meaning in not having a last clause, as opposed to
    // an empty clause.
    last: Option<StatementGroup>,
    area: Span,
}

impl Try {
    pub fn base(&self) -> &StatementGroup {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut StatementGroup {
        &mut self.base
    }

    pub fn catch(&self) -> &Roll<Catch> {
        &self.catch
    }

    pub fn catch_mut(&mut self) -> &mut Roll<Catch> {
        &mut self.catch
    }

    pub fn last(&self) -> &Option<StatementGroup> {
        &self.last
    }

    pub fn last_mut(&mut self) -> &mut Option<StatementGroup> {
        &mut self.last
    }
}

impl Node for Try {
    fn parse(stream: &mut TokenStream) -> Result<Try, Error> {
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
