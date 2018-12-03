use lox::expr::*;
use lox::token::TokenLiteral;

pub struct AstPrinter;

impl<'a> AstPrinter {
    pub fn print(&self, expr: &Expr<'a>) -> String {
        return expr.accept(self);
    }

    pub fn parenthesize(&self, name: &'a str, expr: &[&Expr<'a>]) -> String {
        let mut s = String::from("(") + name;
        
        for &e in expr {
            s += " ";
            s += &e.accept(self);
        }

        s + ")"
    }
}

impl<'a> ExprVisitor<'a> for AstPrinter {
    fn visit_binary(&self, expr: &Binary<'a>) -> String {
        self.parenthesize(expr.operator.lexeme, &[expr.left.as_ref(), expr.right.as_ref()])
    }

    fn visit_grouping(&self, expr: &Grouping<'a>) -> String {
        self.parenthesize("group", &[expr.expression.as_ref()])
    }

    fn visit_literal(&self, expr: &Literal<'a>) -> String {
        match expr.value {
            TokenLiteral::Str(s) => String::from(s),
            TokenLiteral::Number(n) => n.to_string(),
            TokenLiteral::Bool(n) => n.to_string(),
            TokenLiteral::None => String::from("nil"),
        }
    }

    fn visit_unary(&self, expr: &Unary<'a>) -> String {
        self.parenthesize(expr.operator.lexeme, &[expr.right.as_ref()])
    }
}