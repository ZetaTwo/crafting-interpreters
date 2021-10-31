mod lox;

use crate::lox::Lox;
use std::env;

fn main() {
    let args = env::args();
    match args.len() {
        1 => Lox::run_prompt(),
        2 => {
            let script_file = env::args().nth(1).expect("no script file provided");
            Lox::run_file(script_file);
        },
        _ => {
            println!("Usage jlox [script]");
            std::process::exit(exitcode::USAGE);
        }
    }
}
