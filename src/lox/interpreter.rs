use std::rc::*;
use lox::token::*;
use lox::expr::*;

pub struct Interpreter;

type InterpRes<'a> = Result<TokenLiteral, (&'static str, Rc<Token<'a>>)>;

impl<'a> Interpreter {
    fn evaluate(&self, expr: ExprRef<'a, InterpRes<'a>>) -> InterpRes<'a> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: TokenLiteral) -> bool {
        match literal {
            TokenLiteral::None => false,
            TokenLiteral::Bool(b) => b,
            _ => true,
        }
    }

    pub fn interpret<F>(&self, expr: ExprRef<'a, InterpRes<'a>>, mut err: F)
    where F: FnMut(Rc<Token<'a>>, &'static str) {
        match self.evaluate(expr) {
            Result::Ok(token_literal) => {
                println!("{:?}", token_literal);
            },
            Result::Err(e) => {
                err(e.1, e.0);
            }
        }
    }
}

impl<'a> Visitor<'a, InterpRes<'a>> for Interpreter {
    fn visit_binary(&self, expr: &Binary<'a, InterpRes<'a>>) -> InterpRes<'a> {
        let right = self.evaluate(Rc::clone(&expr.right))?;
        let left = self.evaluate(Rc::clone(&expr.left))?;
        Result::Ok(match (left, right) {
            (TokenLiteral::Number(ln), TokenLiteral::Number(rn)) => {
                match expr.operator.token_type {
                    TokenType::Minus => TokenLiteral::Number(ln - rn),
                    TokenType::Plus => TokenLiteral::Number(ln + rn),
                    TokenType::Slash => TokenLiteral::Number(ln / rn),
                    TokenType::Star => TokenLiteral::Number(ln * rn),
                    TokenType::Greater => TokenLiteral::Bool(ln > rn),
                    TokenType::GreaterEqual => TokenLiteral::Bool(ln >= rn),
                    TokenType::Less => TokenLiteral::Bool(ln < rn),
                    TokenType::LessEqual => TokenLiteral::Bool(ln <= rn),
                    TokenType::EqualEqual => TokenLiteral::Bool(ln == rn),
                    TokenType::BangEqual => TokenLiteral::Bool(ln != rn),
                    _ => return Result::Err(("Invalid operator for number type.", Rc::clone(&expr.operator))),
                }
            },
            (TokenLiteral::Str(ls), TokenLiteral::Str(rs)) => {
                match expr.operator.token_type {
                    TokenType::Plus => TokenLiteral::Str(ls + &rs),
                    TokenType::Greater => TokenLiteral::Bool(ls > rs),
                    TokenType::GreaterEqual => TokenLiteral::Bool(ls >= rs),
                    TokenType::Less => TokenLiteral::Bool(ls < rs),
                    TokenType::LessEqual => TokenLiteral::Bool(ls <= rs),
                    TokenType::EqualEqual => TokenLiteral::Bool(ls == rs),
                    TokenType::BangEqual => TokenLiteral::Bool(ls != rs),
                    _ => return Result::Err(("Invalid operator for string type.", Rc::clone(&expr.operator))),
                }
            },
            (TokenLiteral::Bool(ls), TokenLiteral::Bool(rs)) => {
                match expr.operator.token_type {
                    TokenType::EqualEqual => TokenLiteral::Bool(ls == rs),
                    TokenType::BangEqual => TokenLiteral::Bool(ls != rs),
                    _ => return Result::Err(("Invalid operator for boolean type.", Rc::clone(&expr.operator))),
                }
            },
            _ => return Result::Err(("Invalid expression.", Rc::clone(&expr.operator))),
        })
    }

    fn visit_grouping(&self, expr: &Grouping<'a, InterpRes<'a>>) -> InterpRes<'a> {
        self.evaluate(Rc::clone(&expr.expression))
    }

    fn visit_literal(&self, expr: &Literal) -> InterpRes<'a> {
        Result::Ok(expr.value.clone())
    }
    
    fn visit_unary(&self, expr: &Unary<'a, InterpRes<'a>>) -> InterpRes<'a> {
        let right = self.evaluate(Rc::clone(&expr.right))?;

        Result::Ok(match expr.operator.token_type {
            TokenType::Bang => TokenLiteral::Bool(!self.is_truthy(right)),
            TokenType::Minus => {
                match right {
                    TokenLiteral::Number(n) => TokenLiteral::Number(-n),
                    _ => return Result::Err(("Operand must be a number.", Rc::clone(&expr.operator))),
                }
            },
            _ => TokenLiteral::None
        })
    }
}