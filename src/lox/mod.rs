pub mod token;
pub mod scanner;
pub mod expr;
pub mod ast_printer;

use self::scanner::Scanner;

use std::fs;
use std::io;
use std::io::*;
use std::process;

pub struct Lox{
    had_error: bool,
}

impl<'a> Lox {
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

    fn run(&mut self, source: &'a str) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens(|err| self.error(err.0, err.1));
        for token in tokens.iter() {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, whr: &str, message: &str) {
        let s = format!("[line {}] Error{}: {}\n", line, whr, message);
        io::stderr().write(s.as_bytes()).unwrap();

        self.had_error = true;
    }
}