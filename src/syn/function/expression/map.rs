use super::Expression;
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};
use syn::{BasicNode, Node, Roll};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map(Roll<MapPair>);

impl Node for Map {
    fn parse(stream: &mut TokenStream) -> Result<Map> {
        let contents = Roll::with_terminate_trail(
            stream,
            TokenKind::LeftBrace,
            TokenKind::Comma,
            TokenKind::RightBrace,
        )?;
        Ok(Map(contents))
    }
}

impl BasicNode for Map {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPair(Expression, Expression, Span);

impl Node for MapPair {
    fn parse(stream: &mut TokenStream) -> Result<MapPair> {
        let key = Expression::parse(stream)?;
        let mut span = key.span();
        span |= stream.expect_one(TokenKind::Rocket)?.span();
        let value = Expression::parse(stream)?;
        span |= value.span();

        Ok(MapPair(key, value, span))
    }
}

impl BasicNode for MapPair {
    fn span(&self) -> Span {
        self.2
    }
}
