use std::fs;
use std::io;
use std::io::*;
use std::process;
use std::rc::*;

pub mod token;
pub mod scanner;
pub mod expr;
pub mod ast_printer;
pub mod parser;

use self::scanner::*;
use self::token::*;
use self::parser::*;
use self::ast_printer::AstPrinter;

pub struct Lox{
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false,
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            io::stdout().write("> ".as_bytes()).unwrap();
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            self.run(&input);

            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, filename: &str) {
        let source = fs::read_to_string(filename).unwrap();
        self.run(&source);

        if self.had_error {
            process::exit(65);
        }
    }

    fn run<'a>(&mut self, source: &'a str) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens(|line, err| self.error(line, err));
        let mut parser = Parser::new(tokens);
        let expression = parser.parse(|tok, err| self.error_token(tok, err));

        // Stop if there was a syntax error.
        if self.had_error {
            return;
        }

        match expression {
            Some(expr) => {
                let ast_printer = AstPrinter {};
                println!("{}", ast_printer.print(expr.as_ref()));
            },
            _ => {},
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn error_token<'a>(&mut self, token: Rc<Token<'a>>, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn report(&mut self, line: usize, whr: &str, message: &str) {
        let s = format!("[line {}] Error{}: {}\n", line, whr, message);
        io::stderr().write(s.as_bytes()).unwrap();

        self.had_error = true;
    }
}