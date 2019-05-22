//! A module dedicated completely to resolving types.  Essentially, within any
//! given type reference, we need to be able to walk back up the "stack" to
//! determine the best absolute type to use.  For example:
//!
//! ```
//! use Some::T;
//!
//! module A::B {
//!   struct C {
//!     field: T
//!   }
//! }
//! ```
//!
//! For the field in the struct, how do we resolve the type that it references?
//! First, we do type extraction; see [`super::type_`] for more information on
//! that.  The result is that along with the definition for struct `C`, we get
//! a [`super::TypeState`] struct - this contains both the path of the current
//! type definition (`A::B`), as well as all use imports up until that point.
//! In our case, we only have one use import - `Some::T`.
//!
//! So to resolve, we take the current type's partial name (`T`) and first
//! check if it is defined within the current type path (e.g. `A::B::T`).
//! Then, we check each of the `use`s, by matching the imported name against
//! the partial name.  Since we had `Some::T` as an import, `T` is the imported
//! name, which matches the current type's partial name; so we check if
//! `Some::T` is defined.  We keep going through all uses until we have
//! generated a viable set of references, and collecting all of the defined
//! ones.  If we come down with more than one possible type, we can warn the
//! developer - or, if we come down with no possible type, we can throw an
//! error.

use inkwell::types::BasicTypeEnum;
use crate::reduce::{Path, Reduce, TypeState};
use crate::diag::{DiagnosticSync, Name, Span};
use crate::syn::{BasicNode, Type};

pub(super) fn kind<'s>(
    reduce: &mut Reduce<'s>,
    tstate: &TypeState<'s>,
    kind: &'s Type,
) -> Option<BasicTypeEnum> {
    let applicable = generate_possible_references(tstate, kind)
        .flat_map(|(span, typ)| {
            let anno = Path::from_syn(typ, None);
            reduce.types.get_key_value(&anno).map(|(k, v)| (span, k, v))
        })
        .collect::<Vec<_>>();

    if applicable.is_empty() {
        missing_type_error(&reduce.set, tstate, kind);
        return None;
    } else if applicable.len() > 1 {
        ambiguous_type_error(&reduce, &applicable[..], kind);
    }

    Some(*applicable[0].2)
}

fn missing_type_error(set: &DiagnosticSync<'_>, tstate: &TypeState<'_>, kind: &Type) {
    set.emit(
        Name::UnknownType,
        kind.span(),
        format!("unidentified type {}", kind),
    );
    if set.active(Name::UnknownType) {
        for refer in generate_possible_references(tstate, kind) {
            set.emit(
                Name::TypeTrace,
                refer.0,
                "note: type resolution attempted against",
            )
        }
    }
    unimplemented!()
}

fn ambiguous_type_error(
    reduce: &Reduce<'_>,
    applicable: &[(Span, &Path<'_>, &BasicTypeEnum)],
    kind: &Type,
) {
    reduce.set.emit(
        Name::AmbiguousType,
        kind.span(),
        format!("ambiguous type {}", kind),
    );

    if reduce.set.active(Name::AmbiguousType) {
        let first = applicable[0].0;
        reduce
            .set
            .emit(Name::AcceptedType, first, "note: accepted type");
        for given in applicable[1..].iter() {
            reduce
                .set
                .emit(Name::PossibleType, given.0, "note: possible type");
        }
    }
}

fn generate_possible_references<'r, 's: 'r>(
    tstate: &'r TypeState<'s>,
    kind: &'s Type,
) -> impl Iterator<Item = (Span, Vec<&'s Type>)> + 'r {
    tstate
        .base()
        .iter()
        .map(move |t| (t.span(), vec![*t, kind]))
        .chain(
            tstate
                .uses()
                .iter()
                .flat_map(|use_| use_.trails().iter().map(move |trail| (use_, trail)))
                .filter(move |(_, trail)| trail.applies(kind))
                .map(move |(use_, trail)| (trail.span(), trail.combine(use_.prefix(), kind))),
        )
}
