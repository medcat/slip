use syn::function::expression as expr;
use syn::function::statement as stmt;
use syn::function::statement::Statement;

pub trait StatementVisitor {
    type Error;
    type Value;

    fn visit_statement_group(
        &mut self,
        group: &mut stmt::StatementGroup,
    ) -> Result<(), Self::Error> {
        for s in group.iter_mut() {
            self.visit_statement(s)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &mut Statement) -> Result<(), Self::Error> {
        match stmt {
            Statement::If(if_) => self.visit_statement_if(if_),
            Statement::Let(let_) => self.visit_statement_let(let_),
            Statement::Try(try) => self.visit_statement_try(try),
            Statement::For(for_) => self.visit_statement_for(for_),
            Statement::While(while_) => self.visit_statement_while(while_),
            Statement::Unless(unless) => self.visit_statement_unless(unless),
            Statement::Return(return_) => self.visit_statement_return(return_),
            Statement::Expression(expr) => self.visit_expression(expr).map(|_| ()),
        }
    }

    fn visit_statement_if(&mut self, if_: &mut stmt::If) -> Result<(), Self::Error> {
        for cond in if_ {
            cond.condition_mut()
                .as_mut()
                .map(|e| self.visit_expression(e).map(|_| ()))
                .unwrap_or(Ok(()))?;
            self.visit_statement_group(cond.body_mut())?;
        }

        Ok(())
    }

    fn visit_statement_let(&mut self, let_: &mut stmt::Let) -> Result<(), Self::Error> {
        let_.value_mut()
            .as_mut()
            .map(|e| self.visit_expression(e).map(|_| ()))
            .unwrap_or(Ok(()))
    }

    fn visit_statement_try(&mut self, try: &mut stmt::Try) -> Result<(), Self::Error> {
        self.visit_statement_group(try.base_mut())?;
        for catch in try.catch_mut() {
            self.visit_statement_group(catch.body_mut())?;
        }

        Ok(())
    }

    fn visit_statement_for(&mut self, for_: &mut stmt::For) -> Result<(), Self::Error> {
        self.visit_expression(for_.iterator_mut())?;
        self.visit_statement_group(for_.body_mut())
    }

    fn visit_statement_while(&mut self, while_: &mut stmt::While) -> Result<(), Self::Error> {
        self.visit_expression(while_.condition_mut())?;
        self.visit_statement_group(while_.body_mut())
    }

    fn visit_statement_unless(&mut self, unless: &mut stmt::Unless) -> Result<(), Self::Error> {
        self.visit_expression(unless.condition_mut())?;
        self.visit_statement_group(unless.body_mut())
    }

    fn visit_statement_return(&mut self, return_: &mut stmt::Return) -> Result<(), Self::Error> {
        return_
            .value_mut()
            .as_mut()
            .map(|v| self.visit_expression(v).map(|_| ()))
            .unwrap_or(Ok(()))
    }

    fn visit_expression(&mut self, expr: &mut expr::Expression)
        -> Result<Self::Value, Self::Error>;
}
