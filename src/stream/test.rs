use super::*;
use crate::diag::DiagnosticSync;

#[test]
fn it_lexes() {
    let diag = DiagnosticSync::default();
    let source = diag.push("(text)", Some("return 3.default();"));
    let lexer = TokenStream::new("return 3.default();", source, diag);
    let result = lexer
        .map(|r| r.map(|v| (v.kind, v.take_value())).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            (TokenKind::Return, None),
            (TokenKind::Integer, Some("3")),
            (TokenKind::Period, None),
            (TokenKind::Identifier, Some("default")),
            (TokenKind::LeftParen, None),
            (TokenKind::RightParen, None),
            (TokenKind::Semicolon, None),
        ]
    );
}
