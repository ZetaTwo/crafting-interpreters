mod scanner;
mod token;
mod tokentype;
mod literal;

use std::fs;
use std::io::{self, BufRead};
use std::iter;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false
        }
    }

    pub fn run_file(&mut self, script_file: String) {
        println!("Will execute script file {}", script_file);
        let contents = fs::read_to_string(script_file).expect("Unable to read file TODO");
        self.run(contents);
        if self.had_error {
            std::process::exit(exitcode::DATAERR);
        }
    }

    pub fn run_prompt(&mut self) {
        println!("Will execute prompt");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            self.run(line);
            self.had_error = false;
        }
    }

    fn run(&mut self, contents: String) {
        println!("Contents: {}", contents);
        let mut scanner = scanner::Scanner::new(self, contents);
        for token in scanner.scan_tokens() {
            println!("Token: {}", token);
        }
    }

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
