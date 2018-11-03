use crate::syn::function::expression as expr;
use crate::syn::function::statement as stmt;
use crate::syn::function::statement::Statement;

pub trait StatementVisitor {
    type Error;
    type Value;

    fn visit_statement_group(&mut self, group: &stmt::StatementGroup) -> Result<(), Self::Error> {
        for s in group.iter() {
            self.visit_statement(s)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), Self::Error> {
        match stmt {
            Statement::If(if_) => self.visit_statement_if(if_),
            Statement::Let(let_) => self.visit_statement_let(let_),
            Statement::Try(try_) => self.visit_statement_try_(try_),
            Statement::For(for_) => self.visit_statement_for(for_),
            Statement::While(while_) => self.visit_statement_while(while_),
            Statement::Unless(unless) => self.visit_statement_unless(unless),
            Statement::Return(return_) => self.visit_statement_return(return_),
            Statement::Expression(expr) => self.visit_expression(expr).map(|_| ()),
        }
    }

    fn visit_statement_if(&mut self, if_: &stmt::If) -> Result<(), Self::Error> {
        for cond in if_ {
            cond.condition()
                .as_ref()
                .map(|e| self.visit_expression(e).map(|_| ()))
                .unwrap_or(Ok(()))?;
            self.visit_statement_group(cond.body())?;
        }

        Ok(())
    }

    fn visit_statement_let(&mut self, let_: &stmt::Let) -> Result<(), Self::Error> {
        let_.value()
            .as_ref()
            .map(|e| self.visit_expression(e).map(|_| ()))
            .unwrap_or(Ok(()))
    }

    fn visit_statement_try_(&mut self, try_: &stmt::Try) -> Result<(), Self::Error> {
        self.visit_statement_group(try_.base())?;
        for catch in try_.catch() {
            self.visit_statement_group(catch.body())?;
        }

        Ok(())
    }

    fn visit_statement_for(&mut self, for_: &stmt::For) -> Result<(), Self::Error> {
        self.visit_expression(for_.iterator())?;
        self.visit_statement_group(for_.body())
    }

    fn visit_statement_while(&mut self, while_: &stmt::While) -> Result<(), Self::Error> {
        self.visit_expression(while_.condition())?;
        self.visit_statement_group(while_.body())
    }

    fn visit_statement_unless(&mut self, unless: &stmt::Unless) -> Result<(), Self::Error> {
        self.visit_expression(unless.condition())?;
        self.visit_statement_group(unless.body())
    }

    fn visit_statement_return(&mut self, return_: &stmt::Return) -> Result<(), Self::Error> {
        return_
            .value()
            .as_ref()
            .map(|v| self.visit_expression(v).map(|_| ()))
            .unwrap_or(Ok(()))
    }

    fn visit_expression(&mut self, expr: &expr::Expression) -> Result<Self::Value, Self::Error>;
}
