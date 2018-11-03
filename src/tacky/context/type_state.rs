use crate::syn;

pub struct TypeState<'s> {
    pub(crate) base: syn::Type,
    pub(crate) uses: Vec<&'s syn::Use>,
}

impl<'s> TypeState<'s> {
    pub fn base(&self) -> &syn::Type {
        &self.base
    }
    pub fn uses(&self) -> &[&'s syn::Use] {
        &self.uses[..]
    }
}
