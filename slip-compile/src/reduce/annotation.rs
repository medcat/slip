use super::Scope;
use crate::diag::Span;
use crate::reduce::Path;
use crate::syn::{BasicNode, Item};

/// An annotated item - an item, along with the path to the item (type path)
/// and the type imports.  This is easier to keep track of than having these
/// things be separate - plus, we generate the annotation name from this setup,
/// as well.
pub struct Annotation<'s> {
    scope: Scope<'s>,
    item: &'s Item,
}

impl<'s> Annotation<'s> {
    pub fn to_path(&self) -> Path<'s> {
        let mut items = self.scope.base().to_owned();
        let mut fname = None;
        match self.item {
            Item::Enum(enum_) => items.push(enum_.kind()),
            Item::Struct(struct_) => items.push(struct_.kind()),
            Item::Function(func_) => fname = Some(func_.name()),
            _ => {}
        }
        Path::from_syn(items, fname)
    }

    pub fn is_type(&self) -> bool {
        match self.item {
            Item::Enum(_) | Item::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self.item {
            Item::Function(_) => true,
            _ => false,
        }
    }

    pub fn generic_list<'l>(&'l self) -> impl Iterator<Item = &'s crate::syn::Type> + 'l {
        // First, we'll collect all of the `Type`s from the scope, as they may have
        // generics we can use.
        let scope_generics =
            self.scope
                // All of the base types that we're dealing with...
                .base()
                // Iterate over them...
                .iter()
                // And make sure they're only single references.
                .cloned()
                // Now, with these types, we'll extract only the generic component
                // from it, as that's all we care about.
                .filter_map(|v| v.generics().as_ref())
                // Expand that roll out into a proper iterator.
                .flat_map(|roll| roll.iter());
        // We'll then take the list of generics that the item itself has, and
        // tack it on.  Not all items can have generics, and not all items do
        // have generics.
        let list_generics = self.item.generics()
            .into_iter()
            // Expand that roll out into a proper iterator.
            .flat_map(|roll| roll.iter());
        scope_generics.chain(list_generics)
    }

    pub fn item(&self) -> &'s Item {
        self.item
    }

    pub fn scope(&self) -> &Scope<'s> {
        &self.scope
    }
}

impl<'s> From<(Scope<'s>, &'s Item)> for Annotation<'s> {
    fn from((scope, item): (Scope<'s>, &'s Item)) -> Annotation<'s> {
        Annotation { scope, item }
    }
}

impl BasicNode for Annotation<'_> {
    fn span(&self) -> Span {
        self.item.span()
    }
}
