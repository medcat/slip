use super::Key;
use crate::diag::Span;
use serde_derive::*;

mod function;
mod kind;

pub use self::function::{Expression, Function, Statement};
pub use self::kind::{GenericKey, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Definition {
    Module(Span),
    /// There's a bit of implicit information here.  So if you imagine
    /// a structure with the following implicit structure:
    ///
    /// ```
    /// struct Struct {
    ///     generics: [Type]; // an array of types
    ///     elements: [(String, Type)] // an array of string, types
    ///     location: Location
    /// }
    /// ```
    ///
    /// Then this condenses that information down into something useful.
    /// Since we don't actually need to know the values of the generics,
    /// for their types, we can just reference to a generic in this
    /// imaginary array.  Then, we can expand out the element array,
    /// such that we include information about the base type, and any
    /// generic arguments that that base type might have.  The base
    /// type might be one of the struct's generic parameters, in which
    /// case the value of the type is a [`GenericKey::Generic`] that
    /// points to the corresponding entry in the generic array.  The
    /// same applies to the generic arguments - if any of the generic
    /// arguments match the struct's generic parameters, then the
    /// generic index is used - otherwise, a [`GenericKey::Key`] is used,
    /// to denote an instantiable type.
    Struct(Vec<(String, Type)>, Span),

    /// The enum.  This also has a similar set up as structs, but there
    /// are slight differences now.  There are two major types of enums:
    /// UnitEnums, this variant, and ValueEnums, the next variant.  Unit
    /// Enums Have the same element-type pair as earlier, except for one
    /// minor difference.  The "value" in this case can be a collection
    /// of values, as in `A(B, C)`.  We need to handle this, and any
    /// generics that those type values have.
    UnitEnum(Vec<(String, Vec<(Type)>)>, Span),

    /// The enum.  This also has a similar set up as structs, but there
    /// are slight differences now.  There are two major types of enums:
    /// UnitEnums, the previous variant, and ValueEnums, this variant.
    /// This doesn't have any possible generic parameters, and so we
    /// don't have to care about GenericKey, here.  Instead, we have a
    /// Value type.  This is a constant Value derived directly from
    /// the source code.
    ValueEnum(Vec<(String, Expression)>, Span),

    /// A function.  This is the basis for behavior, and is normally the
    /// "entry point" of a program.  The parameters here also may be
    /// genericised.  There are two groups of generics here now - the
    /// ones inherited from e.g. the module, and the ones also defined
    /// on the function.  The first parameter here dictates how many
    /// additional generics the function defines on top of the module.
    /// The third value is the actual function body.  If this is just
    /// a declaration that such a function exists, however, then there
    /// is no function body.
    Function(usize, Vec<(String, Type)>, Option<Function>, Span),
}
