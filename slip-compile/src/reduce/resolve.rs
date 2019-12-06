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

use crate::diag::{DiagnosticSync, Name, Span};
use crate::reduce::{Path, Reduce, Scope, Annotation};
use crate::syn::{BasicNode, Type};
use slip_typal::module::{TypeId, TypeReference};
use slip_typal::spec::ModuleSpec;

pub(super) fn kind<'s>(
    reduce: &mut Reduce<'s>,
    annotation: &Annotation<'s>,
    type_: &'s Type,
) -> TypeReference {
    // first, we're going to check that the incoming type is on our generics
    // list.  There are two components to the generics list: the generics that
    // are passed along the way (located in the `scope`), and the generics that
    // are a part of the type (located in the `type_`).  This will help resolve
    // types that are like e.g. `struct SomeThing<A> { v: A }`.
    if let Some(result) = find_generic(annotation, type_) {
        return result;
    }

    // Now, we try to identify the base type.  Since we know this type isn't
    // a generic (our earlier check failed), we can now check all of the types
    // in scope, to see if they match.
    let base = collect_applicable(reduce, annotation.scope(), type_)
        .unwrap_or_else(|| reduce.module.void_type());

    // The next step is to take all of the generics we've applied to the type,
    // and try to find where that type is located.  This will resolve types
    // like `SomeType<A, String>`; ideally, the first would resolve to the
    // generic `A`, and the second would resolve to the absolute type `String`,
    // resulting in a mixed type reference.
    let generics = type_
        .generics()
        .as_ref()
        .into_iter()
        .flat_map(|r| r.iter())
        .map(|name| {
            // Allows us to resolve the generics/types inside the type.
            kind(reduce, annotation, name)
        })
        .collect::<Vec<_>>();
    if generics.is_empty() {
        // If there were no generics, then we'll just create the absolute
        // reference.
        TypeReference::Absolute(base)
    } else {
        TypeReference::Mix(base, generics)
    }
}

/// This function gives me a headache.
fn find_generic(
    annotation: &Annotation<'_>,
    type_: &Type,
) -> Option<TypeReference> {
    annotation.generic_list()
        // Now, we need to know the position of the generics in the generic
        // list, if it is in there.  We'll do a simple equality check.
        .position(|gen| gen == type_)
        // Cast it to a u64.
        .map(|a| a as u64)
        // And wrap it in a TypeReference.
        .map(TypeReference::Generic)
}

fn collect_applicable<'r, 's: 'r>(
    reduce: &'r mut Reduce<'s>,
    scope: &Scope<'s>,
    type_: &'s Type,
) -> Option<TypeId> {
    // We'll do a short test to see if the incoming type is one of our static
    // types.  This saves us a lot of work, as we just need to consult our
    // table.
    if let Some(id) = get_static_type(reduce, type_) {
        return Some(id);
    }

    // Now we'll generate the possible references that our type can refer to.
    // `generate_possible_references` returns only potential references that
    // the `use` statements may refer to; we'll actually perform the check
    // ourselves by checking our type table.
    let possible_references = generate_possible_references(scope, type_)
        // This reduces our possible reference set to the types that exist.
        // We do the lookup, and attempt to return the type id.  If that
        // succeeds, then we have the type, up to and including the type id.
        .filter_map(|(span, path)| reduce.types.get(&path).map(|id| (span, path, *id)))
        .collect::<Vec<_>>();
    if possible_references.len() > 1 {
        // If we ended up with more than one type, then we're dealing with an
        // ambiguous type scenario; this isn't life-ending for the resolution,
        // but it does mean we may end up picking a sub-optimal type.
        ambiguous_type_error(
            &reduce,
            possible_references.iter().map(|(s, _, _)| *s),
            type_,
        );
    }

    if let Some((_, _, id)) = possible_references.get(0) {
        Some(*id)
    } else  {
        // Since we checked our static types, and we checked the uses and
        // there's nothing, then we'll just have to give up.
        missing_type_error(&reduce.set, scope, type_);
        None
    }
}

fn missing_type_error(set: &DiagnosticSync<'_>, scope: &Scope<'_>, kind: &Type) {
    set.emit(
        Name::UnknownType,
        kind.span(),
        format!("unidentified type {}", kind),
    );
    if set.active(Name::UnknownType) {
        for refer in generate_possible_references(scope, kind) {
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
    mut applicable: impl Iterator<Item = Span>, //&[(Span, &Path<'_>, TypeId)],
    kind: &Type,
) {
    reduce.set.emit(
        Name::AmbiguousType,
        kind.span(),
        format!("ambiguous type {}", kind),
    );

    if reduce.set.active(Name::AmbiguousType) {
        let first = applicable.next().unwrap();
        reduce
            .set
            .emit(Name::AcceptedType, first, "note: accepted type");
        for given in applicable {
            reduce
                .set
                .emit(Name::PossibleType, given, "note: possible type");
        }
    }
}

fn generate_possible_references<'r, 's: 'r>(
    scope: &'r Scope<'s>,
    kind: &'s Type,
) -> impl Iterator<Item = (Span, Path<'s>)> + 'r {
    let in_scope = scope
        .base()
        .iter()
        // We need to look in scope, but we also need to do it incrementally.
        // e.g. if the code looks something like this:
        // ```
        // mod A {
        //   mod B::C {
        //      mod D {
        //        struct E { v: V }
        //     }
        //   }
        // }
        // ```
        // Then we need to look for `V` in `::V`, `A::V`, `A::B::C::V`,
        // and `A::B::C::D::V`.
        .scan(vec![], move |state, typ| {
            state.push(*typ);
            let mut out = state.clone();
            out.push(kind);
            Some((typ.span(), out))
        });
    let use_scope = scope
        // First, list all of the `use`s in scope.
        .uses()
        .iter()
        .cloned()
        .flat_map(move |use_| {
            // Then, for each use, we'll split out the trails.  Each trail
            // will tell us how to import a specific subset of types.
            use_.trails().iter().map(move |trail| (use_, trail))
        })
        // So we need to filter out all of the trails that don't apply
        // to this specific subset.  Basically, if we have `use A::B`,
        // then if we're looking at type `C`, we can safely discard
        // this.
        .filter(move |(_, trail)| trail.applies(kind))
        // Now we know that the use works, so we're now going to combine
        // our type with the prefix from the `use`.  Using the same
        // example as above, we'll attach `B` onto `A`, to make `A::B`;
        // this is more important for renaming uses (`use A::B as C`)
        // or star uses (`use A::*`).
        .map(move |(use_, trail)| (trail.span(), trail.combine(use_.prefix(), kind)));

    in_scope
        .chain(use_scope)
        .map(|(span, types)| (span, Path::from_syn(types, None)))
}

fn void_prime(r: &mut ModuleSpec) -> TypeId {
    r.void_type()
}
fn bool_prime(r: &mut ModuleSpec) -> TypeId {
    r.primitive_type(1)
}
fn i8_prime(r: &mut ModuleSpec) -> TypeId {
    r.primitive_type(8)
}
fn i16_prime(r: &mut ModuleSpec) -> TypeId {
    r.primitive_type(16)
}
fn i32_prime(r: &mut ModuleSpec) -> TypeId {
    r.primitive_type(32)
}
fn i64_prime(r: &mut ModuleSpec) -> TypeId {
    r.primitive_type(64)
}
fn size_prime(r: &mut ModuleSpec) -> TypeId {
    r.size_type()
}
fn ptr_prime(r: &mut ModuleSpec) -> TypeId {
    r.pointer_type()
}

type StaticTypeGen = fn(&mut ModuleSpec) -> TypeId;

static STATIC_TYPES: &[(Path<'static>, StaticTypeGen)] = &[
    (slip_path!(void), void_prime),
    (slip_path!(bool), bool_prime),
    (slip_path!(i8), i8_prime),
    (slip_path!(u8), i8_prime),
    (slip_path!(i16), i16_prime),
    (slip_path!(u16), i16_prime),
    (slip_path!(i32), i32_prime),
    (slip_path!(u32), i32_prime),
    (slip_path!(i64), i64_prime),
    (slip_path!(u64), i64_prime),
    (slip_path!(isize), size_prime),
    (slip_path!(usize), size_prime),
    (slip_path!(["$slip"]::["ptr"]), ptr_prime),
];

fn static_types<'r, 's: 'r>(
    reduce: &'r mut Reduce<'s>,
) -> Vec<(Span, &'static Path<'static>, TypeId)> {
    STATIC_TYPES
        .iter()
        .map(move |(path, act)| {
            let span = Span::identity();
            let id = (*act)(&mut reduce.module);
            (span, path, id)
        })
        .collect()
}

fn get_static_type(reduce: &mut Reduce<'_>, type_: &Type) -> Option<TypeId> {
    if type_.generics().is_some() {
        return None;
    }

    if let Some((_, action)) = STATIC_TYPES.iter().find(|(p, _)| p.eq(type_)) {
        Some(action(&mut reduce.module))
    } else {
        None
    }
}
