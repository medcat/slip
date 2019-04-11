use super::definition::{Expression, GenericKey, Type};
use super::{Definition, Key, Name, Reference};
use crate::diag::{Diagnostic, DiagnosticSet, Span};
use crate::error::*;
use crate::syn::{self, BasicNode};

pub(super) struct Descent<'r, 's: 'r> {
    reference: &'r mut Reference,
    set: &'r mut DiagnosticSet<'s>,
    uses: Vec<Vec<&'r syn::Use>>,
    stack: Vec<&'r syn::Type>,
}

impl<'r, 's> Descent<'r, 's> {
    pub fn new(reference: &'r mut Reference, set: &'r mut DiagnosticSet<'s>) -> Descent<'r, 's> {
        Descent {
            reference,
            set,
            uses: vec![vec![]],
            stack: vec![],
        }
    }

    pub fn descend(&mut self, root: &'r syn::Root) -> Result<()> {
        for item in root.items() {
            self.descend_into(item)?;
        }

        Ok(())
    }

    fn descend_into(&mut self, item: &'r syn::Item) -> Result<()> {
        match item {
            syn::Item::Module(mod_) => self.descend_into_module(mod_),
            syn::Item::Struct(struct_) => self.descend_into_struct(struct_),
            syn::Item::Use(use_) => self.descend_into_use(use_),
            syn::Item::Function(func) => self.descend_into_func(func),
            syn::Item::Enum(enum_) => self.descend_into_enum(enum_),
        }
    }

    fn descend_into_module(&mut self, mod_: &'r syn::Module) -> Result<()> {
        let modtype = mod_.kind();
        self.stack.push(modtype);
        self.uses.push(vec![]);
        for item in mod_.items() {
            self.descend_into(item)?;
        }
        self.uses.pop();
        self.stack.pop();
        Ok(())
    }

    fn descend_into_struct(&mut self, struct_: &'r syn::Struct) -> Result<()> {
        let kind = syn::Type::join_all(self.stack.iter().map(|t| *t).chain(Some(struct_.kind())));
        let name = Name::from(&kind);
        let _key = self.reference.lookup(&name);
        let mut definition = vec![];
        definition.reserve(struct_.elements().len());
        for element in struct_.elements().iter() {
            let name = element.value().value().unwrap().to_owned();
            let kind = normalize(self, &kind, element.kind())?;

            definition.push((name, kind));
        }

        let definition = Definition::Struct(definition, struct_.span());
        // TODO: check if it already has a definition.
        self.reference.define(&name, definition)?;
        Ok(())
    }

    fn descend_into_use(&mut self, use_: &'r syn::Use) -> Result<()> {
        self.uses.last_mut().map(|r| r.push(use_));
        Ok(())
    }

    fn descend_into_func(&mut self, func: &'r syn::Function) -> Result<()> {
        unimplemented!()
    }

    fn descend_into_enum(&mut self, enum_: &'r syn::Enum) -> Result<()> {
        let kind = syn::Type::join_all(self.stack.iter().map(|t| *t).chain(Some(enum_.kind())));
        let name = Name::from(&kind);
        let enum_unit = enum_
            .variants()
            .iter()
            .find(|v| match v {
                syn::EnumVariant::Unit(_, _, _) => true,
                _ => false,
            })
            .is_some();

        if enum_unit {
            self.descend_into_unit_enum(name, &kind, enum_)
        } else {
            self.descend_into_value_enum(name, enum_)
        }
    }

    fn descend_into_value_enum(&mut self, name: Name, enum_: &'r syn::Enum) -> Result<()> {
        let mut counter = 0;
        let mut variants = vec![];
        variants.reserve(enum_.variants().len());

        for variant in enum_.variants().iter() {
            match variant {
                syn::EnumVariant::Value(name, content, _) => {
                    let value = build_const_expr(self, &content)?.unwrap_or(counter);
                    counter = value + 1;
                    variants.push((
                        name.value().unwrap().to_owned(),
                        Expression::Integer(value, name.span()),
                    ));
                }
                syn::EnumVariant::Name(name) => {
                    let value = counter;
                    counter = value + 1;
                    variants.push((
                        name.value().unwrap().to_owned(),
                        Expression::Integer(value, name.span()),
                    ));
                }

                syn::EnumVariant::Unit(_, _, _) => unreachable!(),
            }
        }

        let definition = Definition::ValueEnum(variants, enum_.span());
        self.reference.define(&name, definition)?;

        Ok(())
    }

    fn descend_into_unit_enum(
        &mut self,
        name: Name,
        kind: &syn::Type,
        enum_: &'r syn::Enum,
    ) -> Result<()> {
        let mut variants: Vec<(String, Vec<(Type)>)> = vec![];
        variants.reserve(enum_.variants().len());

        for variant in enum_.variants().iter() {
            match variant {
                syn::EnumVariant::Name(name) => {
                    variants.push((name.value().unwrap().to_owned(), vec![]));

                    // self.reference.define()
                }
                syn::EnumVariant::Unit(name, parts, _) => {
                    let mut collected = vec![];
                    collected.reserve(parts.value().len());

                    for part in parts.value().iter() {
                        collected.push(normalize(self, kind, part)?);
                    }

                    variants.push((name.value().unwrap().to_owned(), collected));
                }
                syn::EnumVariant::Value(_, _, _) => {}
            }
        }

        let definition = Definition::UnitEnum(variants, enum_.span());
        self.reference.define(&name, definition)?;

        Ok(())
    }
}

fn normalize(descent: &mut Descent, top: &syn::Type, given: &syn::Type) -> Result<Type> {
    match (top.generics(), given.generics()) {
        (Some(gen), None) => match gen.iter().position(|a| a == given) {
            Some(i) => return Ok(Type::new(GenericKey::Generic(i), vec![])),
            _ => {}
        },
        _ => {}
    }

    let applicable = descent
        .uses
        .iter()
        .flat_map(|i| i.iter())
        .flat_map(|u| u.trails().iter().map(move |t| (*u, t)))
        .filter(|(use_, trail)| use_trail_applies(descent.reference, use_, trail, given))
        .collect::<Vec<_>>();

    if applicable.len() > 1 {
        descent.set.emit(
            Diagnostic::AmbiguousType,
            given.span(),
            "the given type is ambiguous in the current use set; maybe try removing unneeded uses?"
                .to_string(),
        )?;
        for appl in applicable.iter() {
            descent.set.emit(
                Diagnostic::TypeReference,
                appl.1.span(),
                "another use already defines this type".to_string(),
            )?;
        }
    } else if applicable.len() < 1 {
        descent.set.emit(
            Diagnostic::UnknownType,
            given.span(),
            "the given type is unknown".to_string(),
        )?;
    }

    match applicable.get(0) {
        Some(appl) => {
            let parts = appl
                .0
                .prefix()
                .iter()
                .chain(given.parts().iter())
                .cloned()
                .collect::<Vec<_>>();
            let mut generics = vec![];
            generics.reserve(
                given
                    .generics()
                    .as_ref()
                    .map(|s| s.value().len())
                    .unwrap_or(0),
            );
            for gen in given.generics().iter().flat_map(|i| i.iter()) {
                generics.push(normalize(descent, top, gen)?);
            }
            let span = parts.iter().fold(Span::identity(), |a, e| a | e.span());
            let kind = syn::Type::new(parts, None, span);
            let name = Name::from(&kind);
            let key = descent.reference.lookup(&name);
            Ok(Type::new(GenericKey::Key(key), generics))
        }

        None => Ok(Type::new(GenericKey::Key(Key(0)), vec![])),
    }
}

fn use_trail_applies(
    refer: &Reference,
    use_: &syn::Use,
    trail: &syn::UseTrail,
    given: &syn::Type,
) -> bool {
    match trail.name() {
        Some(n) => n == given.parts(),
        None => {
            // so we've got a star.  We'll have to iterate over what we
            // have to see if we've got something of use... otherwise,
            // nope.
            let combined = use_
                .prefix()
                .iter()
                .chain(given.parts().iter())
                .cloned()
                .collect::<Vec<_>>();
            let span = combined.iter().fold(Span::identity(), |a, e| a | e.span());
            let ctype = syn::Type::new(combined, None, span);
            refer.contains(&Name::from(&ctype))
        }
    }
}

fn build_const_expr(
    descent: &mut Descent,
    expr: &syn::function::expression::Expression,
) -> Result<Option<usize>> {
    use crate::syn::function::expression::Atom;
    use crate::syn::function::expression::Expression as SynExpr;
    match expr {
        SynExpr::Atom(Atom::Integer(value)) => Ok(value
            .value()
            .unwrap()
            .parse::<usize>()
            .map(Some)
            .unwrap_or(None)),
        e => {
            descent.set.emit(
                Diagnostic::NonConstExpr,
                e.span(),
                "the given expression is not constant".to_owned(),
            )?;
            Ok(None)
        }
    }
}
