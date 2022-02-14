use std::io;
use std::io::Write;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn init() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run_file(&mut self, path: &String) {
        if self.had_error {
            std::process::exit(65);
        }
        if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    pub fn run_promp(&mut self) {
        let stdin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            let line = stdin.read_line(&mut buffer);
            match line {
                Ok(0) => break,
                Ok(_) => {
                    self.had_error = false;
                }

                _ => break,
            }
        }
    }

    pub fn run(&mut self) {
        todo!("Implement run function!");
    }

    pub fn error(&mut self, line: i32, message: String) {
        self.report(line, String::from(""), message);
    }

    pub fn report(&mut self, line: i32, where_error: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_error, message);
        self.had_error = true;
    }
        
}
