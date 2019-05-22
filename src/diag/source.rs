use std::borrow::Cow;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(pub(super) usize);

#[derive(Debug, Clone)]
pub struct Source<'c> {
    pub(super) id: SourceId,
    pub(super) name: Cow<'c, str>,
    pub(super) content: Option<Cow<'c, str>>,
}

impl<'c> Source<'c> {
    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> Option<&str> {
        self.content.as_ref().map(AsRef::as_ref)
    }
}
