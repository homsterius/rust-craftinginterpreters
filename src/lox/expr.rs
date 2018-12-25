use std::rc::*;
use lox::token::Token;
use lox::token::TokenLiteral;

pub type ExprRef<'a, T> = Rc<Expr<'a, T> + 'a>;

pub trait Visitor<'a, T: 'a> {
    fn visit_binary(&self, expr: &Binary<'a, T>) -> T;
    fn visit_grouping(&self, expr: &Grouping<'a, T>) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
    fn visit_unary(&self, expr: &Unary<'a, T>) -> T;
}

pub trait Expr<'a, T: 'a> {
    fn accept(&self, visitor: &Visitor<'a, T>) -> T;
}


pub struct Binary<'a, T: 'a> {
    pub left: ExprRef<'a, T>,
    pub operator: Rc<Token<'a>>,
    pub right: ExprRef<'a, T>,
}

impl<'a, T: 'a> Binary<'a, T> {
    pub fn new(left: ExprRef<'a, T>, operator: Rc<Token<'a>>, right: ExprRef<'a, T>) -> ExprRef<'a, T> {
        Rc::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl<'a, T> Expr<'a, T> for Binary<'a, T> {
    fn accept(&self, visitor: &Visitor<'a, T>) -> T {
        (*visitor).visit_binary(self)
    }
}


pub struct Grouping<'a, T: 'a> {
    pub expression: ExprRef<'a, T>,
}

impl<'a, T: 'a> Grouping<'a, T> {
    pub fn new(expression: ExprRef<'a, T>) -> ExprRef<'a, T> {
        Rc::new(Grouping { expression })
    }
}

impl<'a, T> Expr<'a, T> for Grouping<'a, T> {
    fn accept(&self, visitor: &Visitor<'a, T>) -> T {
        (*visitor).visit_grouping(self)
    }
}


pub struct Literal {
    pub value: TokenLiteral,
}

impl<'a> Literal {
    pub fn new<T: 'a>(value: TokenLiteral) -> ExprRef<'a, T> {
        Rc::new(Literal { value })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Literal {
    fn accept(&self, visitor: &Visitor<'a, T>) -> T {
        (*visitor).visit_literal(self)
    }
}


pub struct Unary<'a, T: 'a> {
    pub operator: Rc<Token<'a>>,
    pub right: ExprRef<'a, T>,
}

impl<'a, T: 'a> Unary<'a, T> {
    pub fn new(operator: Rc<Token<'a>>, right: ExprRef<'a, T>) -> ExprRef<'a, T> {
        Rc::new(Unary {
            operator,
            right,
        })
    }
}

impl<'a, T: 'a> Expr<'a, T> for Unary<'a, T> {
    fn accept(&self, visitor: &Visitor<'a, T>) -> T {
        (*visitor).visit_unary(self)
    }
}