mod ast;
mod lexer;
mod parser;
mod runtime;

use lexer::tokenize;
use parser::parse;
use runtime::execute;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: pwalang <file.pwa>");
        process::exit(1);
    }

    let file_path = &args[1];

    if !file_path.ends_with(".pwa") {
        eprintln!("Error: Expected a .pwa file");
        process::exit(1);
    }

    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read '{file_path}': {error}");
            process::exit(1);
        }
    };

    let tokens = match tokenize(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer error: {error}");
            process::exit(1);
        }
    };

    let simulation = match parse(&tokens) {
        Ok(simulation) => simulation,
        Err(error) => {
            eprintln!("Parser error: {error}");
            process::exit(1);
        }
    };

    execute(&simulation);
}
