use super::{FunctionId, UnconstrainedType};
use crate::stream::Token;
use crate::syn;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Name<'t> {
    Data(UnconstrainedType<'t>),
    RawData(Vec<Cow<'t, str>>),
    Fn(FunctionId<'t>),
    RawFn(Vec<Cow<'t, str>>, Cow<'t, str>, usize),
}

impl<'t> Name<'t> {
    pub fn to_static(&self) -> Name<'static> {
        match self {
            Name::Data(a) => Name::Data(a.to_static()),
            Name::RawData(a) => {
                Name::RawData(a.iter().map(|v| Cow::Owned(v.to_string())).collect())
            }
            Name::Fn(a) => Name::Fn(a.to_static()),
            Name::RawFn(a, b, c) => Name::RawFn(
                a.iter().map(|v| Cow::Owned(v.to_string())).collect(),
                Cow::Owned(b.to_string()),
                *c,
            ),
        }
    }

    pub fn to_raw(&'t self) -> Name<'t> {
        match self {
            Name::Data(a) => Name::RawData(
                a.kind()
                    .parts()
                    .iter()
                    .map(|tok| Cow::Borrowed(tok.value().unwrap()))
                    .collect(),
            ),
            Name::RawData(a) => {
                Name::RawData(a.iter().map(|i| Cow::Owned(i.to_string())).collect())
            }
            Name::Fn(fid) => Name::RawFn(
                fid.kind()
                    .kind()
                    .parts()
                    .iter()
                    .map(|part| Cow::Borrowed(part.value().unwrap()))
                    .collect(),
                Cow::Owned(fid.name().value().to_string()),
                fid.arity(),
            ),
            Name::RawFn(a, b, c) => Name::RawFn(
                a.iter().map(|i| Cow::Owned(i.to_string())).collect(),
                Cow::Owned(b.to_string()),
                *c,
            ),
        }
    }
}

impl<'t> PartialEq<UnconstrainedType<'t>> for Name<'t> {
    fn eq(&self, other: &UnconstrainedType<'t>) -> bool {
        match self {
            Name::Data(a) => a == other,
            Name::RawData(a) => other
                .kind()
                .parts()
                .iter()
                .map(|part| part.value().unwrap())
                .eq(a.iter().map(|part| &*part)),
            Name::Fn(_) => false,
            Name::RawFn(_, _, _) => false,
        }
    }
}

impl<'t> PartialEq<syn::Type> for Name<'t> {
    fn eq(&self, other: &syn::Type) -> bool {
        match self {
            Name::Data(a) => a == other,
            Name::RawData(a) => other
                .parts()
                .iter()
                .map(|part| part.value().unwrap())
                .eq(a.iter().map(|part| &*part)),
            Name::Fn(_) => false,
            Name::RawFn(_, _, _) => false,
        }
    }
}

impl<'t> PartialEq<FunctionId<'t>> for Name<'t> {
    fn eq(&self, other: &FunctionId<'t>) -> bool {
        match self {
            Name::Data(_) => false,
            Name::RawData(_) => false,
            Name::Fn(a) => a == other,
            Name::RawFn(a, b, c) => {
                a.iter().map(|part| &*part).eq(other
                    .kind()
                    .kind()
                    .parts()
                    .iter()
                    .map(|part| part.value().unwrap()))
                    && other.name().value() == &*b
                    && *c == other.arity()
            }
        }
    }
}

impl<'t> From<&'t syn::Type> for Name<'t> {
    fn from(kind: &'t syn::Type) -> Name<'t> {
        Name::Data(UnconstrainedType::from(kind))
    }
}

impl<'t> From<Vec<&'t str>> for Name<'t> {
    fn from(parts: Vec<&'t str>) -> Name<'t> {
        Name::RawData(parts.into_iter().map(Cow::Borrowed).collect())
    }
}
