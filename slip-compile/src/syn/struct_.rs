use super::{BasicNode, Node, Roll, Type};
use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    name: Type,
    elements: Roll<StructElement>,
    area: Span,
}

impl Struct {
    pub fn kind(&self) -> &Type {
        &self.name
    }
    pub fn elements(&self) -> &[StructElement] {
        &self.elements.value()
    }
}

impl Node for Struct {
    fn parse(stream: &mut TokenStream) -> Result<Struct, Error> {
        let mut span = stream.expect_one(TokenKind::Struct)?.span();
        let kind = Type::parse(stream)?;
        span |= kind.span();
        let contents = Roll::with_terminate_trail_once(
            stream,
            TokenKind::LeftBrace,
            TokenKind::Comma,
            TokenKind::RightBrace,
        )?;
        span |= contents.span();

        Ok(Struct {
            name: kind,
            elements: contents,
            area: span,
        })
    }
}

impl BasicNode for Struct {
    fn span(&self) -> Span {
        self.area
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructElement {
    name: Token,
    kind: Type,
    area: Span,
}

impl StructElement {
    pub fn value(&self) -> &Token {
        &self.name
    }
    pub fn kind(&self) -> &Type {
        &self.kind
    }
}

impl Node for StructElement {
    fn parse(stream: &mut TokenStream) -> Result<StructElement, Error> {
        let name = stream.expect_one(TokenKind::Identifier)?;
        let colon = stream.expect_one(TokenKind::Colon)?;
        let kind = Type::parse(stream)?;
        let span = name.span() | colon.span() | kind.span();

        Ok(StructElement {
            name,
            kind,
            area: span,
        })
    }
}

impl BasicNode for StructElement {
    fn span(&self) -> Span {
        self.area
    }
}
