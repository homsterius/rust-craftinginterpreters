extern crate lox;

use std::env;
use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            println!("Usage: lox [script]");
            std::process::exit(64);
        },
    }
}