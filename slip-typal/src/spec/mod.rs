use crate::module::{Module, Name, Type, TypeDefinition, TypeId, TypeReference};
use crate::version::Version;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModuleSpec {
    module: Module,
    primitives: HashMap<u64, TypeId>,
    pointer: Option<TypeId>,
    size: Option<TypeId>,
}

impl ModuleSpec {
    pub fn new(name: impl Into<String>, version: Version) -> ModuleSpec {
        let mod_ = Module {
            name: name.into(),
            version,
            requirements: vec![],
            types: Default::default(),
            funcs: Default::default(),
        };
        ModuleSpec {
            module: mod_,
            primitives: HashMap::new(),
            pointer: None,
            size: None,
        }
    }

    pub fn primitive_type(&mut self, size: u64) -> TypeId {
        self.primitives
            .get(&size)
            .cloned()
            .unwrap_or_else(|| self.generate_primitive(size))
    }

    pub fn type_push(&mut self, type_: Type) -> TypeId {
        let id = self.module.next_type_id();
        self.module.types.insert(id, type_);
        id
    }

    pub fn void_type(&mut self) -> TypeId {
        self.primitive_type(0)
    }

    pub fn pointer_type(&mut self) -> TypeId {
        self.pointer.unwrap_or_else(|| self.generate_pointer())
    }

    pub fn size_type(&mut self) -> TypeId {
        self.size.unwrap_or_else(|| self.generate_size())
    }

    pub fn stub_type<N, G>(&mut self, name: N, generics: G) -> TypeId
    where
        N: Into<Name>,
        G: IntoIterator<Item = Name>,
    {
        let type_ = Type {
            name: name.into(),
            generics: generics.into_iter().collect(),
            definition: TypeDefinition::Stub,
        };

        self.type_push(type_)
    }

    pub fn update_type<F>(&mut self, id: TypeId, f: F)
    where
        F: FnOnce(&mut Type),
    {
        if let Some(ty) = self.module.types.get_mut(&id) {
            f(ty);
        }
    }

    pub fn struct_type<N, G, E>(&mut self, name: N, generics: G, elements: E) -> TypeId
    where
        N: Into<Name>,
        G: IntoIterator<Item = Name>,
        E: IntoIterator<Item = (String, TypeReference)>,
    {
        let type_ = Type {
            name: name.into(),
            generics: generics.into_iter().collect(),
            definition: TypeDefinition::Struct(elements.into_iter().collect()),
        };

        self.type_push(type_)
    }

    fn generate_primitive(&mut self, size: u64) -> TypeId {
        let type_ = Type {
            name: format!("i{}", size).into(),
            generics: vec![],
            definition: TypeDefinition::Primitive(size),
        };

        let id = self.type_push(type_);
        self.primitives.insert(size, id);
        id
    }

    fn generate_pointer(&mut self) -> TypeId {
        let type_ = Type {
            name: ["$slip", "ptr"].iter().cloned().collect(),
            generics: vec!["T".into()],
            definition: TypeDefinition::PrimitivePtr,
        };

        let id = self.type_push(type_);
        self.pointer = Some(id);
        id
    }

    fn generate_size(&mut self) -> TypeId {
        let type_ = Type {
            name: "isize".into(),
            generics: vec![],
            definition: TypeDefinition::PrimitiveSize,
        };

        let id = self.type_push(type_);
        self.size = Some(id);
        id
    }
}
