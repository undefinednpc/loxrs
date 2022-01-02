mod scanner;

use std::io::{Write, stdin, stdout};
use crate::scanner::Scanner;

fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        run(input);
    }
}

#[allow(dead_code, unused_variables)]
fn run_file(path: String) {
    todo!();
}

#[allow(dead_code, unused_variables)]
fn run(source: &str) {
    let tokens = Scanner::scan_tokens(source);
    for token in tokens {
        println!("{}", token);
    }
}

fn main() {
    run_prompt();
}