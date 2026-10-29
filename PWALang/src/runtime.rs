use crate::ast::{Simulation, Statement, Value};

pub fn execute(simulation: &Simulation) {
    for line in collect_output(simulation) {
        println!("{line}");
    }
}

pub fn collect_output(simulation: &Simulation) -> Vec<String> {
    let mut output = Vec::new();

    collect_statements(&simulation.statements, &mut output);

    output
}

fn collect_statements(statements: &[Statement], output: &mut Vec<String>) {
    for statement in statements {
        match statement {
            Statement::Output(value) => {
                output.push(format_value(value));
            }

            Statement::Block(block) => {
                collect_statements(&block.statements, output);
            }

            Statement::Property(_) => {}
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Number(number) => number.to_string(),
        Value::String(text) => text.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    #[test]
    fn executes_output_in_source_order() {
        let source = r#"
            simulation "Output_Test" {
                output("Hello, PWALang!");
                output(42);

                plasma {
                    output("Nested output");
                }
            }
        "#;

        let tokens = tokenize(source).unwrap();
        let simulation = parse(&tokens).unwrap();

        assert_eq!(
            collect_output(&simulation),
            vec!["Hello, PWALang!", "42", "Nested output",]
        );
    }
}
