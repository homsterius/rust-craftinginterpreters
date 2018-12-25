use lox::expr::*;
use lox::token::TokenLiteral;

pub struct AstPrinter;

impl<'a> AstPrinter {
    pub fn print(&self, expr: &Expr<'a, String>) -> String {
        return expr.accept(self);
    }

    pub fn parenthesize(&self, name: &'a str, expr: &[&Expr<'a, String>]) -> String {
        let mut s = String::from("(") + name;
        
        for &e in expr {
            s += " ";
            s += &e.accept(self);
        }

        s + ")"
    }
}

impl<'a> Visitor<'a, String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary<'a, String>) -> String {
        self.parenthesize(expr.operator.lexeme, &[expr.left.as_ref(), expr.right.as_ref()])
    }

    fn visit_grouping(&self, expr: &Grouping<'a, String>) -> String {
        self.parenthesize("group", &[expr.expression.as_ref()])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        match expr.value {
            TokenLiteral::Str(ref s) => s.clone(),
            TokenLiteral::Number(n) => n.to_string(),
            TokenLiteral::Bool(n) => n.to_string(),
            TokenLiteral::None => String::from("nil"),
        }
    }

    fn visit_unary(&self, expr: &Unary<'a, String>) -> String {
        self.parenthesize(expr.operator.lexeme, &[expr.right.as_ref()])
    }
}