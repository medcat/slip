use crate::syn::{Item, Module, Root, Type, Use};

/// Information about the current position in the type structure of the
/// associated item.  This includes the current path to the item (i.e. module
/// names), and all of the `use`s along the path to it.  This should be
/// calculated once after the syntax tree is created, as the syntax tree
/// does not change.
///
/// This borrows from the syntax tree with lifetime `'s`.
pub struct Scope<'s> {
    base: Vec<&'s Type>,
    uses: Vec<&'s Use>,
}

struct Stack<'s> {
    typ_: Vec<&'s Type>,
    use_: Vec<Vec<&'s Use>>,
}

impl<'s> Scope<'s> {
    /// Creates a new typestate from a root structure.  This assumes that the
    /// top level has no type information, i.e. there is no module above this
    /// root.  I don't think we'll ever have the root be nested within another
    /// module, but...
    pub fn build(root: &'s Root) -> Vec<(Scope<'s>, &'s Item)> {
        let mut stack = Stack::new();
        root.items()
            .iter()
            .flat_map(|item| stack.visit(item))
            .collect::<Vec<_>>()
    }

    /// Grabs the base type of this typestate.  This is tied to the lifetime
    /// of the stack because it is constructed from all of the module type
    /// names along the way.  The span on this type is from the last module
    /// name that it touched; for example, if the outer module is `A::B`, and
    /// the inner one is `C::D`, then the span of this type is of the span of
    /// `C::D`, in order to prevent wildly odd spans from occurring.
    pub fn base(&self) -> &[&'s Type] {
        &self.base
    }

    /// Grabs all of the uses that were listed along the path to the current
    /// item.  The outer slice is tied to the lifetime of this struct, while
    /// the inner values are tied to the lifetime of the syntax tree.
    pub fn uses(&self) -> &[&'s Use] {
        &self.uses[..]
    }
}

impl<'s> Stack<'s> {
    fn new() -> Stack<'s> {
        Stack {
            typ_: vec![],
            use_: vec![vec![]],
        }
    }

    fn push(&mut self, typ: &'s Type) {
        self.typ_.push(typ);
        self.use_.push(vec![]);
    }

    fn pop(&mut self) {
        self.use_.pop();
        self.typ_.pop();
    }

    fn collapse(&self) -> Scope<'s> {
        // let mut base = Type::join_all(self.typ_.iter().cloned());
        // *base.span_mut() = self
        //     .typ_
        //     .last()
        //     .map(|s| s.span())
        //     .unwrap_or_else(Span::identity);
        let base = self.typ_.clone();
        let uses = self.use_.iter().flatten().cloned().collect::<Vec<_>>();
        Scope { base, uses }
    }

    fn visit(&mut self, item: &'s Item) -> Box<dyn Iterator<Item = (Scope<'s>, &'s Item)> + 's> {
        match item {
            Item::Enum(_) => self.visit_basic(item),
            Item::Function(_) => self.visit_basic(item),
            Item::Module(mod_) => self.visit_mod(mod_),
            Item::Struct(_) => self.visit_basic(item),
            Item::Use(use_) => self.visit_use(use_),
        }
    }

    fn visit_basic(
        &mut self,
        item: &'s Item,
    ) -> Box<dyn Iterator<Item = (Scope<'s>, &'s Item)> + 's> {
        Box::new(std::iter::once((self.collapse(), item)))
    }

    fn visit_mod(
        &mut self,
        mod_: &'s Module,
    ) -> Box<dyn Iterator<Item = (Scope<'s>, &'s Item)> + 's> {
        self.push(mod_.kind());
        let result = mod_
            .items()
            .iter()
            .flat_map(|item| self.visit(item))
            .collect::<Vec<_>>();
        self.pop();
        Box::new(result.into_iter())
    }

    fn visit_use(&mut self, use_: &'s Use) -> Box<dyn Iterator<Item = (Scope<'s>, &'s Item)> + 's> {
        self.use_.last_mut().unwrap().push(use_);
        Box::new(std::iter::empty())
    }
}
