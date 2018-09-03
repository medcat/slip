use super::*;
use std::borrow::Cow;

#[test]
fn it_lexes() {
    let lexer = TokenStream::new("return 3.default();");
    let result = lexer
        .map(|r| r.map(|v| (v.kind, Cow::Owned(v.value.unwrap()))).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            (TokenKind::Return, Cow::Borrowed("return")),
            (TokenKind::Integer, Cow::Borrowed("3")),
            (TokenKind::Period, Cow::Borrowed(".")),
            (TokenKind::Identifier, Cow::Borrowed("default")),
            (TokenKind::LeftParen, Cow::Borrowed("(")),
            (TokenKind::RightParen, Cow::Borrowed(")")),
            (TokenKind::Semicolon, Cow::Borrowed(";")),
        ]
    );
}
