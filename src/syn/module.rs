use super::{BasicNode, Item, Node, Type};
use diag::Span;
use error::*;
use stream::{TokenKind, TokenStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module(Type, Vec<Item>, Span);

impl Node for Module {
    fn parse(stream: &mut TokenStream) -> Result<Module> {
        let mut span = stream.expect_one(TokenKind::Module)?.span();
        let kind = Type::parse(stream)?;
        span |= kind.span();
        span |= stream.expect_one(TokenKind::LeftBrace)?.span();

        let mut contents = vec![];

        while !stream.peek_one(TokenKind::RightBrace) {
            let item = Item::parse(stream)?;
            span |= item.span();
            contents.push(item);
        }

        span |= stream.expect_one(TokenKind::RightBrace)?.span();
        Ok(Module(kind, contents, span))
    }
}

impl BasicNode for Module {
    fn span(&self) -> Span {
        self.2
    }
}
