use crate::diag::Span;
use crate::error::*;
use crate::stream::{Token, TokenKind, TokenStream};

mod enum_;
pub mod function;
pub mod item;
mod kind;
mod module;
mod roll;
mod root;
mod struct_;
mod unit;
mod use_;

pub use self::enum_::{Enum, EnumVariant};
pub use self::function::Function;
pub use self::item::Item;
pub use self::kind::Type;
pub use self::module::Module;
pub use self::roll::Roll;
pub use self::root::Root;
pub use self::struct_::{Struct, StructElement};
pub use self::unit::Unit;
pub use self::use_::{Use, UseTrail};

pub trait BasicNode: Sized {
    fn span(&self) -> Span;
}

pub trait Node: BasicNode {
    fn parse(stream: &mut TokenStream) -> Result<Self, Error>;
}

pub fn of(source: &str) -> Result<Root, Error> {
    let set = crate::diag::DiagnosticSync::default();
    let file = set.push("(implicit)", Some(source));
    let mut token_stream = TokenStream::new(source, file, set);
    Root::parse(&mut token_stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const BASIC_SOURCE: &str = r#"
use Slip::List;

module Some::Program {
    fn some_func(): Slip::Int {
        return 42;
    }
}"#;

    #[bench]
    fn bench_basic_parse(b: &mut Bencher) {
        b.iter(|| of(BASIC_SOURCE).unwrap())
    }

    #[test]
    fn test_basic_parse() {
        let _ = of(BASIC_SOURCE).unwrap();
    }
}
