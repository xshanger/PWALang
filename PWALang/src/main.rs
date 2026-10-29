mod ast;
mod lexer;
mod parser;
mod runtime;

use lexer::tokenize;
use parser::parse;
use runtime::execute;

fn main() {
    let source = r#"
    simulation "Output_Test" {
        output("Hello, PWALang!");
        output(42);
        output(1e17);

        plasma {
            density = 1e17;
            output("Plasma configuration loaded");
        }

        beam {
            energy = 1.0;
            output("Beam configuration loaded");
        }
    }"#;

    let tokens = match tokenize(source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer error: {error}");
            std::process::exit(1);
        }
    };

    let simulation = match parse(&tokens) {
        Ok(simulation) => simulation,
        Err(error) => {
            eprintln!("Parser error: {error}");
            std::process::exit(1);
        }
    };

    execute(&simulation);
}
