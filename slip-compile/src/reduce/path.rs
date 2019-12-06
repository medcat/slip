use crate::stream::Token;
use crate::syn::function::FunctionName;
use crate::syn::Type;
use slip_typal::module::Name;
use std::borrow::Cow;
use std::fmt;

//#[derive(Debug, Clone, PartialEq, Eq, Hash)]
//#[doc(hidden)]
//pub enum StaticDynamic<'s> {
//    Static(&'static [&'static str]),
//    Dynamic(Vec<&'s str>),
//}

#[macro_export]
macro_rules! slip_path {
    ($($i:ident)::*) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed(stringify!($i))),*]), fname: None }
    };
    ($($i:ident)::*.$v:ident) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed(stringify!($i))),*]), fname: Some(std::borrow::Cow::Borrowed(stringify!($v))) }
    };
    ($($i:ident)::*.[$v:expr]) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed(stringify!($i))),*]), fname: Some(std::borrow::Cow::Borrowed($v)) }
    };
    ($([$i:expr])::*) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed($i)),*]), fname: None }
    };
    ($([$i:expr])::*.$v:ident) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed($i)),*]), fname: Some(std::borrow::Cow::Borrowed(stringify!($v))) }
    };
    ($([$i:expr])::*.[$v:expr]) => {
        Path { base: std::borrow::Cow::Borrowed(&[$(std::borrow::Cow::Borrowed($i)),*]), fname: Some(std::borrow::Cow::Borrowed($v)) }
    };
}

/// The path of an item.  Since the actual type path may vary in
// various references, we have to do a few things to handle this.
/// First, we say that `A::B` is a continuous path; next, if a module `C` is
/// defined within `A::B`, then we say that `[A::B, C]` is the completed path.
/// `[A::B, C]` is defined to be equivalent to any combination of continuous
/// paths, as long as the components are, in order, `A`, `B`, and `C`; in
/// words, `[A::B, C]` is equal to `[A, B::C]`, `[A::B::C]`, and `[A, B, C]`.
/// However, since they are all represented differently in terms of the type
/// structure, we store the completed path as an array here, and do a flat-map
/// on the parts to determine equality.
///
/// Note that this also takes into account function names, if the type
/// definition is a function.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path<'s> {
    #[doc(hidden)]
    pub base: Cow<'s, [Cow<'s, str>]>,
    #[doc(hidden)]
    pub fname: Option<Cow<'s, str>>,
}

impl<'s> Path<'s> {
    pub fn new(base: Vec<Cow<'s, str>>, fname: Option<Cow<'s, str>>) -> Path<'s> {
        Path {
            base: Cow::Owned(base),
            fname,
        }
    }

    pub fn to_name(&self) -> Name {
        self.base
            .iter()
            .map(Cow::as_ref)
            .chain(self.fname.as_ref().map(Cow::as_ref).into_iter())
            .collect()
    }

    pub fn from_syn(ty_: Vec<&'s Type>, fname: Option<&'s FunctionName>) -> Path<'s> {
        let base = ty_
            .iter()
            .flat_map(|ty| ty.parts().iter())
            .flat_map(|tok| tok.value())
            .map(Cow::Borrowed)
            .collect::<Vec<_>>();
        let name = fname.map(|s| s.value()).map(Cow::Borrowed);
        Self::new(base, name)
    }

    pub fn with_fname(self, fname: Option<impl Into<Cow<'s, str>>>) -> Path<'s> {
        Path {
            base: self.base,
            fname: fname.map(Into::into),
        }
    }

    pub fn is_func(&self) -> bool {
        self.fname.is_some()
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let joined = self.base.join("::");
        if let Some(fname) = self.fname.as_ref() {
            write!(f, "{}.{}", joined, fname)
        } else {
            joined.fmt(f)
        }
    }
}

impl PartialEq<Type> for Path<'_> {
    fn eq(&self, other: &Type) -> bool {
        other.generics().is_none() && !self.is_func() && {
            let ours = self.base.iter().map(|s| s.as_ref());
            let theirs = other.parts().iter().flat_map(Token::value);
            ours.eq(theirs)
        }
    }
}
