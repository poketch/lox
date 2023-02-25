use crate::{
    error::LoxError,
    expression::{self, Expr, Visitor},
};

pub struct ASTPrinter;

impl ASTPrinter {

    pub fn new() -> Self {
        Self
    }

    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept::<String>(self)
    }


    fn parenthesize(&self, name: impl Into<String>, exprs: Vec<&Expr>) -> Result<String, LoxError> {
        
        let mut out = String::from("(");

        out = format!("{}{}", out, name.into());
        for expr in exprs {

            out.push_str(" ");
            out.push_str(&expr.accept::<String>(self)?);

        }
        out.push(')');

        Ok(out) 
    }
}

impl Visitor<String> for ASTPrinter {
    fn visit_binary_expr(
        &self,
        expr: &expression::BinaryExpr,
    ) -> Result<String, crate::error::LoxError> {
        self.parenthesize(expr.operator().lexeme(), vec![expr.left(), expr.right()])
    }

    fn visit_grouping_expr(
        &self,
        expr: &expression::GroupingExpr,
    ) -> Result<String, crate::error::LoxError> {
        self.parenthesize("group", vec![expr.expression()])
    }

    fn visit_literal_expr(
        &self,
        expr: &expression::LiteralExpr,
    ) -> Result<String, crate::error::LoxError> {
        if let Some(value) = expr.value() {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(
        &self,
        expr: &expression::UnaryExpr,
    ) -> Result<String, crate::error::LoxError> {
        self.parenthesize(expr.operator().lexeme(), vec![expr.right()])
    }
}
