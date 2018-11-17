use crate::syn;
use crate::syn::function::FunctionName;
use serde_derive::*;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnconstrainedType<'t>(Cow<'t, syn::Type>);

impl<'t> UnconstrainedType<'t> {
    pub fn kind(&self) -> &syn::Type {
        &*self.0
    }

    pub fn into_owned(self) -> UnconstrainedType<'static> {
        UnconstrainedType(Cow::Owned(self.0.into_owned()))
    }

    pub fn to_static(&self) -> UnconstrainedType<'static> {
        UnconstrainedType(Cow::Owned(self.0.clone().into_owned()))
    }
}

impl<'t> PartialEq for UnconstrainedType<'t> {
    fn eq(&self, other: &Self) -> bool {
        self.0.parts().iter().map(|part| part.value()).eq(other
            .0
            .parts()
            .iter()
            .map(|part| part.value()))
    }
}

impl<'t> PartialEq<syn::Type> for UnconstrainedType<'t> {
    fn eq(&self, other: &syn::Type) -> bool {
        self.0
            .parts()
            .iter()
            .map(|part| part.value())
            .eq(other.parts().iter().map(|part| part.value()))
    }
}

impl<'t> Eq for UnconstrainedType<'t> {}

impl<'t> Hash for UnconstrainedType<'t> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.parts().hash(hasher)
    }
}

impl From<syn::Type> for UnconstrainedType<'static> {
    fn from(kind: syn::Type) -> UnconstrainedType<'static> {
        UnconstrainedType(Cow::Owned(kind))
    }
}

impl<'t> From<&'t syn::Type> for UnconstrainedType<'t> {
    fn from(kind: &'t syn::Type) -> UnconstrainedType<'t> {
        UnconstrainedType(Cow::Borrowed(kind))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionId<'t>(UnconstrainedType<'t>, Cow<'t, FunctionName>, usize);

impl<'t> FunctionId<'t> {
    pub fn new(kind: &'t syn::Type, name: &'t FunctionName, arity: usize) -> FunctionId<'t> {
        FunctionId(kind.into(), Cow::Borrowed(name), arity)
    }

    pub fn from(kind: syn::Type, name: FunctionName, arity: usize) -> FunctionId<'static> {
        FunctionId(kind.into(), Cow::Owned(name), arity)
    }

    pub fn into_owned(self) -> FunctionId<'static> {
        FunctionId(self.0.into_owned(), Cow::Owned(self.1.into_owned()), self.2)
    }

    pub fn to_static(&self) -> FunctionId<'static> {
        FunctionId(
            self.0.to_static(),
            Cow::Owned(self.1.clone().into_owned()),
            self.2,
        )
    }

    pub fn kind(&self) -> &UnconstrainedType<'t> {
        &self.0
    }

    pub fn name(&self) -> &FunctionName {
        &self.1
    }

    pub fn arity(&self) -> usize {
        self.2
    }
}
