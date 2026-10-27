mod ast;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;

fn main() {
    let source = r#"
        simulation "PWFA_Test" {
            density = 1e17;
            description = "Baseline plasma";
        }
    "#;

    println!("PWALang Compiler v0.1.0");
    println!("------------------------");

    let tokens = match tokenize(source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer error: {error}");
            std::process::exit(1);
        }
    };

    match parse(&tokens) {
        Ok(simulation) => {
            println!("Parsing successful!");
            println!("{simulation:#?}");
        }
        Err(error) => {
            eprintln!("Parser error: {error}");
            std::process::exit(1);
        }
    }
}
