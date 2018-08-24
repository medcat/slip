use super::Span;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    file: String,
    span: Span,
}

impl Source {
    pub fn new<'a, T>(file: T, span: Span) -> Source
    where
        T: Into<Cow<'a, str>>,
    {
        Source {
            file: file.into().into_owned(),
            span,
        }
    }
}
