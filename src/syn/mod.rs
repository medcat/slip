use diag::Span;
use error::*;
use stream::{Token, TokenKind, TokenStream};

mod enum_;
pub mod function;
pub mod item;
mod kind;
mod module;
mod roll;
mod struct_;
mod unit;
mod use_;

pub use self::enum_::{Enum, EnumVariant};
pub use self::function::Function;
pub use self::item::Item;
pub use self::kind::Type;
pub use self::module::Module;
pub use self::roll::Roll;
pub use self::struct_::{Struct, StructElement};
pub use self::unit::Unit;

pub trait BasicNode: Sized {
    fn span(&self) -> Span;
}

pub trait Node: BasicNode {
    fn parse(stream: &mut TokenStream) -> Result<Self>;
}