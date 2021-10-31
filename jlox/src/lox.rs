use std::fs;
use std::io::{self, BufRead};

mod scanner;

pub struct Lox {}

impl Lox {
    pub fn run_file(script_file: String) {
        println!("Will execute script file {}", script_file);
        let contents = fs::read_to_string(script_file).expect("Unable to read file TODO");
        Self::run(contents);
    }

    pub fn run_prompt() {
        println!("Will execute prompt");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            Self::run(line);
        }
    }

    fn run(contents: String) {
        println!("Contents: {}", contents);
        let scanner = scanner::Scanner::new(contents);
        for token in scanner.scan_tokens() {
            println!("Token: {}", token);
        }
    }
}
