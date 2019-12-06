use super::Context;
use super::Resolve;
use crate::diag::{Diagnostic, DiagnosticSet};
use crate::error::Error;
use crate::pass::context::TypeState;
use crate::syn;
use crate::syn::function::expression as expr;
use crate::syn::function::statement as stmt;
use crate::syn::BasicNode;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, VoidType};
use inkwell::values::FunctionValue;
use syn::function::FunctionParameter;

pub(super) struct Func<'t, 'm: 't, 's: 't> {
    pub(super) resolv: &'t mut Resolve<'m, 's>,
    pub(super) context: &'t Context<'s>,
    pub(super) tst: &'t TypeState<'s>,
    pub(super) func: &'s syn::Function,
    pub(super) set: &'t mut DiagnosticSet<'t>,
}

impl<'t, 'm: 't, 's: 't> Func<'t, 'm, 's> {
    pub(super) fn create(&mut self) -> Result<FunctionValue, Error> {
        let name = if self.tst.base.is_empty() {
            self.func.name().value().to_owned()
        } else {
            let mut name = self.tst.base.to_string();
            name.reserve(self.func.name().value().len() + 1);
            name += ".";
            name += self.func.name().value();
            name
        };

        let rettype = self
            .func
            .retval()
            .as_ref()
            .map(|r| self.type_of(r))
            .transpose()?;
        let params = try_map(
            self.func.parameters().iter(),
            Some(self.func.parameters().len()),
            |param| match param {
                FunctionParameter::Static(_, ty) => self.type_of(ty),
                FunctionParameter::Ignore(_, ty) => self.type_of(ty),
                FunctionParameter::This(_) => self.type_of(&self.tst.base),
            },
        )?;
        let fn_type = ret_type(rettype, &params[..]);
        let mut fn_value = self.resolv.module.add_function(&name, fn_type, None);

        if self.func.body().is_some() {
            self.build(&mut fn_value)?;
        }

        Ok(fn_value)
    }

    fn build(&mut self, value: &mut FunctionValue) -> Result<(), Error> {
        let group = self.func.body().as_ref().unwrap();
        let rettype = self
            .func
            .retval()
            .as_ref()
            .unwrap_or_else(|| syn::Type::void());

        let head = if group.len() > 1 {
            Some(&group.statements()[0..(group.len() - 2)])
        } else {
            None
        };
        let tail = group.statements().last();

        unimplemented!()
    }

    fn type_of(&mut self, typ: &syn::Type) -> Result<BasicTypeEnum, Error> {
        let find =
            lookup(typ, self.tst.uses.iter().cloned()).find_map(|ty| self.resolv.ty.get(&ty));

        match find {
            Some(ty) => Ok(*ty),
            None => {
                self.set.emit(
                    Diagnostic::UnknownType,
                    typ.span(),
                    format!("could not find type {}", typ),
                )?;
                if self.set.active(Diagnostic::UnknownType) {
                    for i in lookup(typ, self.tst.uses.iter().cloned()) {
                        self.set.emit(
                            Diagnostic::Note,
                            typ.span(),
                            format!("attempted lookup {}", i),
                        )?;
                    }
                }

                Ok(self
                    .resolv
                    .module
                    .get_context()
                    .struct_type(&[], false)
                    .as_basic_type_enum())
            }
        }
    }
}

fn lookup<'s, 't: 's, I: Iterator<Item = &'s syn::Use> + 't>(
    ty: &'s syn::Type,
    iter: I,
) -> impl Iterator<Item = syn::Type> + 's {
    std::iter::once(ty.clone()).chain(iter.flat_map(move |use_| {
        use_.trails().iter().flat_map(move |trail| {
            if trail.applies(ty) {
                Some(trail.combine(use_.prefix(), ty))
            } else {
                None
            }
        })
    }))
}

fn ret_type(basic: Option<BasicTypeEnum>, params: &[BasicTypeEnum]) -> FunctionType {
    match basic {
        Some(base) => base.fn_type(params, false),
        None => VoidType::void_type().fn_type(params, false),
    }
}

fn try_map<InItem, OutItem, OutError, In, Func>(
    mut inv: In,
    len: Option<usize>,
    mut func: Func,
) -> Result<Vec<OutItem>, OutError>
where
    In: Iterator<Item = InItem>,
    Func: FnMut(InItem) -> Result<OutItem, OutError>,
{
    let mut outv = len.map(Vec::with_capacity).unwrap_or_else(Vec::new);
    inv.try_fold(outv, |mut acc, el| {
        func(el).map(|r| {
            acc.push(r);
            acc
        })
    })
}
