use std::rc::*;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {                                   
    // Single-character tokens.                      
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, Str, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: TokenLiteral,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: TokenLiteral, line: usize) -> Rc<Token<'a>> {
        Rc::new(Token {
            token_type,
            lexeme,
            literal,
            line,
        })
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}, line: {}", self.token_type, self.lexeme, self.literal, self.line)
    }
}