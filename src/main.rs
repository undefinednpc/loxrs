mod lox;
mod scanner;
mod token;
mod tokentype;

use crate::lox::Lox;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        let source: &String = &args[1];
        todo!("Run file");
    } else {
        todo!("Run prompt");
    }
}
