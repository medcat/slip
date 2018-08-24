use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    parts: Vec<Token>,
    generics: Option<Roll<Type>>,
    area: Span,
}

impl Node for Type {
    fn parse(stream: &mut TokenStream) -> Result<Type> {
        let start = stream.expect_one(TokenKind::ModuleName)?;
        let mut span = start.span();
        let mut contents = vec![start];

        while stream.peek_one(TokenKind::DoubleColon) {
            span |= stream.expect_one(TokenKind::DoubleColon)?.span();
            let result = stream.expect_one(TokenKind::ModuleName)?;
            span |= result.span();
            contents.push(result);
        }

        let generics = if stream.peek_one(TokenKind::LessThan) {
            Some(Roll::with_terminate_once(
                stream,
                TokenKind::LessThan,
                TokenKind::Comma,
                TokenKind::GreaterThan,
            )?)
        } else {
            None
        };

        generics.as_ref().map(|v| span |= v.span());

        Ok(Type {
            parts: contents,
            generics,
            area: span,
        })
    }
}

impl BasicNode for Type {
    fn span(&self) -> Span {
        self.area
    }
}
