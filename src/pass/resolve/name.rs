use crate::syn;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name<'s>(Vec<&'s str>, Option<&'s str>);

impl<'s> Name<'s> {
    pub(super) fn from_func(func: &'s syn::Function, path: &'s syn::Type) -> Name<'s> {
        let base: Vec<&'s str> = path.parts().iter().flat_map(|s| s.value()).collect();
        let name = func.name().value();
        Name(base, Some(name))
    }
}
