use std::rc::*;
use lox::token::*;
use lox::expr::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Rc<Token<'a>>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Rc<Token<'a>>>) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse<F>(&mut self, mut err: F) -> Option<ExprRef<'a>>
    where F: FnMut(Rc<Token<'a>>, &'static str) {
        match self.expression() {
            Ok(token) => Option::Some(token),
            Err(e) => {
                err(e.0, e.1);
                Option::None
            },
        }
    }

    fn expression(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        self.equalty()
    }

    fn equalty(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        let mut expr = self.comparison()?;

        while self.mtch(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator, right);
        }

        Result::Ok(expr)
    }

    fn comparison(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        let mut expr: Rc<Expr<'a>> = self.addition()?;

        while self.mtch(&[TokenType::Greather, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Rc<Token<'a>> = self.previous();
            let right: Rc<Expr<'a>> = self.addition()?;
            expr = Binary::new(expr, operator, right);
        }

        Result::Ok(expr)
    }

    fn addition(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        let mut expr = self.multiplication()?;

        while self.mtch(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Binary::new(expr, operator, right);
        }

        Result::Ok(expr)
    }

    fn multiplication(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        let mut expr = self.unary()?;

        while self.mtch(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Binary::new(expr, operator, right);
        }

        Result::Ok(expr)
    }

    fn unary(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        if self.mtch(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Result::Ok(Unary::new(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<ExprRef<'a>, (Rc<Token<'a>>, &'static str)> {
        if self.mtch(&[TokenType::False]) {
            return Result::Ok(Literal::new(TokenLiteral::Bool(false)));
        }
        if self.mtch(&[TokenType::True]) {
            return Result::Ok(Literal::new(TokenLiteral::Bool(true)));
        }
        if self.mtch(&[TokenType::Nil]) {
            return Result::Ok(Literal::new(TokenLiteral::None));
        }

        if self.mtch(&[TokenType::Number, TokenType::Str]) {
            return Result::Ok(Literal::new(self.previous().literal.clone()));
        }

        if self.mtch(&[TokenType::LeftParen]) {
            let expr: Rc<Expr<'a>> = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Result::Ok(Grouping::new(expr));
        }

        Result::Err((self.peek(), "Expected expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &'static str) -> Result<Rc<Token<'a>>, (Rc<Token<'a>>, &'static str)> {
        if self.check(token_type) {
            return Result::Ok(self.advance());
        }

        Result::Err((self.peek(), message))
    }

    // fn synchronize(&mut self) {
    //     self.advance();

    //     while !self.is_at_end() {
    //         if self.previous().token_type == TokenType::Semicolon {
    //             return;
    //         }
    //     }

    //     match self.peek().token_type {
    //         TokenType::Class |
    //         TokenType::Fun |
    //         TokenType::Var |
    //         TokenType::For |
    //         TokenType::If |
    //         TokenType::While |
    //         TokenType::Print |
    //         TokenType::Return => return,
    //         _ => {},
    //     }

    //     self.advance();
    // }

    fn mtch(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Rc<Token<'a>> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Rc<Token<'a>> {
        Rc::clone(&self.tokens[self.current])
    }

    fn previous(&self) -> Rc<Token<'a>> {
        Rc::clone(&self.tokens[self.current - 1])
    }
}