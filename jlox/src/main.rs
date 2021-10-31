mod lox;

use crate::lox::Lox;
use std::env;

fn main() {
    let args = env::args();
    let mut lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => {
            let script_file = env::args().nth(1).expect("no script file provided");
            lox.run_file(script_file);
        },
        _ => {
            println!("Usage jlox [script]");
            std::process::exit(exitcode::USAGE);
        }
    }
}
