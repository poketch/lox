use crate::{
    error::LoxError,
    token::{Object, Token},
};

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(be) => be.accept(visitor),
            Expr::Grouping(ge) => ge.accept(visitor),
            Expr::Literal(le) => le.accept(visitor),
            Expr::Unary(ue) => ue.accept(visitor),
            _ => Err(LoxError::new(
                42069,
                "Error Parsing Unknown Expression Type",
            )),
        }
    }
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn left(&self) -> &Expr {
        &*self.left
    }

    pub fn right(&self) -> &Expr {
        &*self.right
    }

    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }

    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

pub struct LiteralExpr {
    value: Option<Object>,
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }

    pub fn value(&self) -> &Option<Object> {
        &self.value
    }

    pub fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Expr {
        &*self.right
    }

    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}
pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}
