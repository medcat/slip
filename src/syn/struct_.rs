use super::{BasicNode, Node, Roll, Type};
use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct(Type, Roll<StructElement>, Span);

impl Node for Struct {
    fn parse(stream: &mut TokenStream) -> Result<Struct> {
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

        Ok(Struct(kind, contents, span))
    }
}

impl BasicNode for Struct {
    fn span(&self) -> Span {
        self.2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructElement(Token, Type, Span);

impl Node for StructElement {
    fn parse(stream: &mut TokenStream) -> Result<StructElement> {
        let name = stream.expect_one(TokenKind::Identifier)?;
        let colon = stream.expect_one(TokenKind::Colon)?;
        let kind = Type::parse(stream)?;
        let span = name.span() | colon.span() | kind.span();

        Ok(StructElement(name, kind, span))
    }
}

impl BasicNode for StructElement {
    fn span(&self) -> Span {
        self.2
    }
}
