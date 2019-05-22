use crate::syn::function::FunctionName;
use crate::syn::Type;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub enum StaticDynamic<'s> {
    Static(&'static [&'static str]),
    Dynamic(Vec<&'s str>),
}

#[macro_export]
macro_rules! slip_path {
    ($($i:ident)::*) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$(stringify!($i)),*]), fname: None }
    };
    ($($i:ident)::*.$v:ident) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$(stringify!($i)),*]), fname: Some(stringify!($v)) }
    };
    ($($i:ident)::*.[$v:expr]) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$(stringify!($i)),*]), fname: Some($v) }
    };
    ($([$i:expr])::*) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$($i),*]), fname: None }
    };
    ($([$i:expr])::*.$v:ident) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$($i),*]), fname: Some(stringify!($v)) }
    };
    ($([$i:expr])::*.[$v:expr]) => {
        Path { base: $crate::reduce::path::StaticDynamic::Static(&[$($i),*]), fname: Some($v) }
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
    pub base: StaticDynamic<'s>,
    #[doc(hidden)]
    pub fname: Option<&'s str>,
}

impl<'s> Path<'s> {
    pub fn new(base: Vec<&'s str>, fname: Option<&'s str>) -> Path<'s> {
        Path {
            base: StaticDynamic::Dynamic(base),
            fname,
        }
    }

    pub fn from_syn(ty_: Vec<&'s Type>, fname: Option<&'s FunctionName>) -> Path<'s> {
        let base = ty_
            .iter()
            .flat_map(|ty| ty.parts().iter())
            .flat_map(|tok| tok.value())
            .collect::<Vec<_>>();
        let name = fname.map(|s| s.value());
        Self::new(base, name)
    }

    pub fn with_fname(self, fname: Option<&'s str>) -> Path<'s> {
        Path {
            base: self.base,
            fname,
        }
    }

    pub fn base(&self) -> &[&'s str] {
        match &self.base {
            StaticDynamic::Dynamic(ref v) => &v[..],
            StaticDynamic::Static(v) => v,
        }
    }

    // we can do this because references implement copy.
    pub fn fname(&self) -> Option<&'s str> {
        self.fname
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let joined = self.base().join("::");
        if let Some(fname) = self.fname {
            write!(f, "{}.{}", joined, fname)
        } else {
            joined.fmt(f)
        }
    }
}
