mod lexer;

use lexer::tokenize;

fn main() {
    let source = r#"
        simulation "PWFA_Test" {
            density = 1e17;
        }
    "#;

    println!("PWALang Compiler v0.1.0");
    println!("------------------------");

    match tokenize(source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{token:?}");
            }
        }
        Err(error) => {
            eprintln!("Lexer error: {error}");
            std::process::exit(1);
        }
    }
}
