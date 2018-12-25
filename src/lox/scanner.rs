use std::rc::*;
use lox::token::Token;
use lox::token::TokenType;
use lox::token::TokenLiteral;

use std::collections::HashMap;

lazy_static! {
    static ref Keywords: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Rc<Token<'a>>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens<F>(&mut self, mut err: F) -> &Vec<Rc<Token<'a>>>
    where F: FnMut(usize, &str) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                err(e.0, e.1);
            }
        }

        self.tokens.push(Token::new(TokenType::Eof, "", TokenLiteral::None, self.line));
        &(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), (usize, &'static str)> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen, TokenLiteral::None),
            ')' => self.add_token(TokenType::RightParen, TokenLiteral::None),
            '{' => self.add_token(TokenType::LeftBrace, TokenLiteral::None),
            '}' => self.add_token(TokenType::RightBrace, TokenLiteral::None),
            ',' => self.add_token(TokenType::Comma, TokenLiteral::None),
            '.' => self.add_token(TokenType::Dot, TokenLiteral::None),
            '-' => self.add_token(TokenType::Minus, TokenLiteral::None),
            '+' => self.add_token(TokenType::Plus, TokenLiteral::None),
            ';' => self.add_token(TokenType::Semicolon, TokenLiteral::None),
            '*' => self.add_token(TokenType::Star, TokenLiteral::None),
            '!' => {
                let m = if self.mtch("=") {TokenType::BangEqual} else {TokenType::Bang};
                self.add_token(m, TokenLiteral::None);
            },
            '=' => {
                let m = if self.mtch("=") {TokenType::EqualEqual} else {TokenType::Equal};
                self.add_token(m, TokenLiteral::None);
            },
            '<' => {
                let m = if self.mtch("=") {TokenType::LessEqual} else {TokenType::Less};
                self.add_token(m, TokenLiteral::None);
            },
            '>' => {
                let m = if self.mtch("=") {TokenType::GreaterEqual} else {TokenType::Greater};
                self.add_token(m, TokenLiteral::None)
            },
            '/' => {
                if self.mtch("/") {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, TokenLiteral::None);
                }
            },
            '"' => self.string()?,
            '\n' => {
                self.line += 1;
            },
            l if Scanner::is_digit(l) => {
                self.number();
            },
            l if Scanner::is_alpha(l) => {
                self.identifier();
            },
            ' ' | '\r' | '\t' => {},
            _ => {
                return Err((self.line, "Unexpected character."));
            },
        }

        Result::Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType, literal: TokenLiteral) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        (&self.source[self.current - 1..self.current]).parse::<char>().unwrap()
    }

    fn mtch(&mut self, expected: &'a str) -> bool {
        if self.is_at_end() {
            return false;
        }

        let lexeme = &self.source[self.current..self.current + 1];
        if lexeme != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        (&self.source[self.current..self.current + 1]).parse().unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        (&self.source[self.current + 1..self.current + 2]).parse().unwrap()
    }

    fn string(&mut self) -> Result<(), (usize, &'static str)> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err((self.line, "Unterminated string."));
        }

        self.advance();

        let s = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::Str, TokenLiteral::Str(String::from(s)));

        Ok(())
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let parsed_literal = (&self.source[self.start..self.current]).parse::<f64>().unwrap();
        self.add_token(
            TokenType::Number,
            TokenLiteral::Number(parsed_literal)
        );
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' ||
        c >= 'A' && c <= 'Z' ||
        c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let txt = &self.source[self.start..self.current];
        match Keywords.get(txt) {
            Some(token) => {
                self.add_token(token.clone(), TokenLiteral::Str(String::from(txt)));
            },
            None => {
                self.add_token(TokenType::Identifier, TokenLiteral::None);
            },
        }
    }
}