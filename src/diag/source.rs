#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceId(usize);

#[derive(Debug, Clone)]
pub struct Source {
    pub(super) id: SourceId,
    pub(super) name: String,
    pub(super) content: String
}

impl Source {
    pub fn id(&self) -> SourceId { self.id }

    pub fn name(&self) -> &str { &self.name }

    pub fn content(&self) -> &str { &self.content }
}