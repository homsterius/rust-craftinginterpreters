use lox::token::Token;
use lox::token::TokenLiteral;


pub trait ExprVisitor<'a, T: 'a> {
    fn visit_binary(&self, expr: &Binary<'a, T>) -> T;
    fn visit_grouping(&self, expr: &Grouping<'a, T>) -> T;
    fn visit_literal(&self, expr: &Literal<'a>) -> T;
    fn visit_unary(&self, expr: &Unary<'a, T>) -> T;
}

pub trait Expr<'a, T: 'a> {
    fn accept(&self, visitor: &ExprVisitor<'a, T>) -> T;
}


pub struct Binary<'a, T: 'a> {
    pub left: Box<Expr<'a, T>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a, T>>,
}

impl<'a, T: 'a> Binary<'a, T> {
    pub fn new(left: Box<Expr<'a, T>>, operator: Token<'a>, right: Box<Expr<'a, T>>) -> Box<Binary<'a, T>> {
        Box::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Binary<'a, T> {
    fn accept(&self, visitor: &ExprVisitor<'a, T>) -> T {
        (*visitor).visit_binary(self)
    }
}


pub struct Grouping<'a, T: 'a> {
    pub expression: Box<Expr<'a, T>>,
}

impl<'a, T: 'a> Grouping<'a, T> {
    pub fn new(expression: Box<Expr<'a, T>>) -> Box<Grouping<'a, T>> {
        Box::new(Grouping { expression })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Grouping<'a, T> {
    fn accept(&self, visitor: &ExprVisitor<'a, T>) -> T {
        (*visitor).visit_grouping(self)
    }
}


pub struct Literal<'a> {
    pub value: TokenLiteral<'a>,
}

impl<'a> Literal<'a> {
    pub fn new(value: TokenLiteral<'a>) -> Box<Literal> {
        Box::new(Literal { value })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Literal<'a> {
    fn accept(&self, visitor: &ExprVisitor<'a, T>) -> T {
        (*visitor).visit_literal(self)
    }
}


pub struct Unary<'a, T: 'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a, T>>,
}

impl<'a, T: 'a> Unary<'a, T> {
    pub fn new(operator: Token<'a>, right: Box<Expr<'a, T>>) -> Box<Unary<'a, T>> {
        Box::new(Unary {
            operator,
            right,
        })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Unary<'a, T> {
    fn accept(&self, visitor: &ExprVisitor<'a, T>) -> T {
        (*visitor).visit_unary(self)
    }
}