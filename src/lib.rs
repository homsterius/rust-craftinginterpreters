#[macro_use]
extern crate lazy_static;

pub mod lox;
pub use lox::Lox;


#[cfg(test)]
mod tests {
    use lox::ast_printer::AstPrinter;
    use lox::expr::*;
    use lox::token::*;

    #[test]
    fn ast_printer() {
        let expression = Binary::new(
            Unary::new(
                Token::new(TokenType::Minus, "-", TokenLiteral::None, 1),
                Literal::new(TokenLiteral::Number(123.0))
            ),
            Token::new(TokenType::Star, "*", TokenLiteral::None, 1),
            Grouping::new(Literal::new(TokenLiteral::Number(45.67)))
        );

        let ast_printer = AstPrinter {};
        assert_eq!("(* (- 123) (group 45.67))", ast_printer.print(expression.as_ref()));
    }
}