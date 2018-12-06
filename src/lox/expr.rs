use std::rc::*;
use lox::token::Token;
use lox::token::TokenLiteral;

pub type ExprRef<'a> = Rc<Expr<'a> + 'a>;

pub trait Visitor<'a, T> {
    fn visit_binary(&self, expr: &Binary<'a>) -> T;
    fn visit_grouping(&self, expr: &Grouping<'a>) -> T;
    fn visit_literal(&self, expr: &Literal<'a>) -> T;
    fn visit_unary(&self, expr: &Unary<'a>) -> T;
}

pub trait Expr<'a> {
    fn accept(&self, visitor: &Visitor<'a, String>) -> String;
}


pub struct Binary<'a> {
    pub left: ExprRef<'a>,
    pub operator: Rc<Token<'a>>,
    pub right: ExprRef<'a>,
}

impl<'a> Binary<'a> {
    pub fn new(left: ExprRef<'a>, operator: Rc<Token<'a>>, right: ExprRef<'a>) -> ExprRef<'a> {
        Rc::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl<'a> Expr<'a> for Binary<'a> {
    fn accept(&self, visitor: &Visitor<'a, String>) -> String {
        (*visitor).visit_binary(self)
    }
}


pub struct Grouping<'a> {
    pub expression: ExprRef<'a>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: ExprRef<'a>) -> ExprRef<'a> {
        Rc::new(Grouping { expression })
    }
}

impl<'a> Expr<'a> for Grouping<'a> {
    fn accept(&self, visitor: &Visitor<'a, String>) -> String {
        (*visitor).visit_grouping(self)
    }
}


pub struct Literal<'a> {
    pub value: TokenLiteral<'a>,
}

impl<'a> Literal<'a> {
    pub fn new(value: TokenLiteral<'a>) -> ExprRef<'a> {
        Rc::new(Literal { value })
    }
}

impl<'a> Expr<'a> for Literal<'a> {
    fn accept(&self, visitor: &Visitor<'a, String>) -> String {
        (*visitor).visit_literal(self)
    }
}


pub struct Unary<'a> {
    pub operator: Rc<Token<'a>>,
    pub right: ExprRef<'a>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Rc<Token<'a>>, right: ExprRef<'a>) -> ExprRef<'a> {
        Rc::new(Unary {
            operator,
            right,
        })
    }
}

impl<'a> Expr<'a> for Unary<'a> {
    fn accept(&self, visitor: &Visitor<'a, String>) -> String {
        (*visitor).visit_unary(self)
    }
}