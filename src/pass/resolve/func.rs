use crate::diag::DiagnosticSet;
use crate::error::Error;
use crate::pass::context::TypeState;
use crate::syn;
use crate::syn::function::expression as expr;
use crate::syn::function::statement as stmt;
use std::cell::RefCell;
use std::collections::HashMap;

pub(super) struct Func {
    pub(super) name: String,
    pub(super) start: usize,
    pub(super) arity: usize,
    pub(super) blocks: Vec<Block>,
}

pub(super) struct Block {
    pub(super) isns: Vec<Instruction>,
}

pub(super) struct Instruction {}

impl Func {
    fn new(name: String, arity: usize) -> Func {
        Func {
            name,
            start: 0,
            arity,
            blocks: vec![],
        }
    }

    pub(super) fn try_from<'s>(
        func: &'s syn::Function,
        ts: &TypeState<'s>,
        set: &mut DiagnosticSet<'_>,
    ) -> Result<Func, Error> {
        let name = format!(
            "{}.{}",
            ts.base
                .parts()
                .iter()
                .flat_map(|p| p.value())
                .collect::<Vec<_>>()
                .join("::"),
            func.name().value()
        );

        let mut fp = FuncParse::new(set);

        for (i, param) in func.parameters().iter().enumerate() {
            match param {
                syn::function::FunctionParameter::Static(tok, ty) => {
                    fp.vars.insert(tok.value().unwrap(), (i, ty));
                }
                syn::function::FunctionParameter::This(tok) => {
                    fp.vars.insert(tok.value().unwrap(), (i, &ts.base));
                }
            }
        }

        fp.top = func.parameters().len();

        let body = func.body().as_ref().unwrap();

        fp.handle_statement_group(body);

        unimplemented!()
    }
}

impl Default for Block {
    fn default() -> Block {
        Block { isns: vec![] }
    }
}

struct FuncParse<'f, 's: 'f> {
    blocks: Vec<RefCell<Block>>,
    current: usize,
    vars: HashMap<&'s str, (usize, &'s syn::Type)>,
    set: &'f mut DiagnosticSet<'s>,
    top: usize,
}

impl<'f, 's: 'f> FuncParse<'f, 's> {
    fn new(set: &'f mut DiagnosticSet<'s>) -> FuncParse<'f, 's> {
        FuncParse {
            blocks: vec![],
            current: 0,
            vars: HashMap::new(),
            set,
            top: 0,
        }
    }

    fn push_var(&mut self, name: &'s str, ty: &'s syn::Type) -> (usize, &'s syn::Type) {
        *self.vars.entry(name).or_insert_with(|| {
            let v = self.top;
            self.top += 1;
            (v, ty)
        })
    }

    fn handle_statement_group(&mut self, gr: &'s stmt::StatementGroup) {
        for statement in gr.iter() {
            match statement {
                stmt::Statement::Expression(ex) => {}
                stmt::Statement::Let(let_) => self.handle_let(let_),
            }
        }
    }

    fn handle_let(&mut self, let_: &'s stmt::Let) {
        let_.token().value();
    }

    fn handle_expression(&mut self, expr: &'s expr::Expression) -> (usize, &'s syn::Type) {}
}
