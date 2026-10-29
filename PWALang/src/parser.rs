use crate::ast::{Block, Property, Simulation, Statement, Value};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<Simulation, String> {
    let mut parser = Parser {
        tokens,
        position: 0,
    };

    parser.parse_simulation()
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn current(&self) -> Option<&'a Token> {
        self.tokens.get(self.position)
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.position + 1)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.current() {
            Some(token) if token == expected => {
                self.advance();
                Ok(())
            }
            other => Err(format!("Expected {expected:?}, found {other:?}")),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.current() {
            Some(Token::Identifier(value)) => {
                let name = value.clone();
                self.advance();
                Ok(name)
            }
            other => Err(format!("Expected identifier, found {other:?}")),
        }
    }

    fn parse_simulation(&mut self) -> Result<Simulation, String> {
        self.expect(&Token::Identifier("simulation".into()))?;

        let name = match self.current() {
            Some(Token::StringLiteral(value)) => {
                let name = value.clone();
                self.advance();
                name
            }
            other => {
                return Err(format!("Expected simulation name, found {other:?}"));
            }
        };

        self.expect(&Token::LeftBrace)?;

        let statements = self.parse_statements()?;

        self.expect(&Token::RightBrace)?;

        if self.current().is_some() {
            return Err("Unexpected tokens after simulation".into());
        }

        Ok(Simulation { name, statements })
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        loop {
            match self.current() {
                Some(Token::RightBrace) => break,

                None => {
                    return Err("Unclosed block".into());
                }

                Some(Token::Identifier(_)) => {
                    statements.push(self.parse_statement()?);
                }

                other => {
                    return Err(format!("Expected statement, found {other:?}"));
                }
            }
        }

        Ok(statements)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        let value = match self.current() {
            Some(Token::Number(number)) => Value::Number(*number),

            Some(Token::StringLiteral(text)) => Value::String(text.clone()),

            other => {
                return Err(format!("Expected number or string, found {other:?}"));
            }
        };

        self.advance();
        Ok(value)
    }

    fn parse_output(&mut self) -> Result<Statement, String> {
        self.expect(&Token::Identifier("output".into()))?;
        self.expect(&Token::LeftParen)?;

        let value = self.parse_value()?;

        self.expect(&Token::RightParen)?;
        self.expect(&Token::Semicolon)?;

        Ok(Statement::Output(value))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::Equals) => Ok(Statement::Property(self.parse_property()?)),

            Some(Token::LeftBrace) => Ok(Statement::Block(self.parse_block()?)),

            Some(Token::LeftParen) => match self.current() {
                Some(Token::Identifier(name)) if name == "output" => self.parse_output(),

                other => Err(format!("Unknown function or statement: {other:?}")),
            },

            other => Err(format!(
                "Expected '=', '{{', or '(' after identifier, found {other:?}"
            )),
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let name = self.parse_identifier()?;

        self.expect(&Token::LeftBrace)?;

        let statements = self.parse_statements()?;

        self.expect(&Token::RightBrace)?;

        Ok(Block { name, statements })
    }

    fn parse_property(&mut self) -> Result<Property, String> {
        let name = self.parse_identifier()?;

        self.expect(&Token::Equals)?;

        let value = match self.current() {
            Some(Token::Number(number)) => {
                let result = Value::Number(*number);
                self.advance();
                result
            }

            Some(Token::StringLiteral(text)) => {
                let result = Value::String(text.clone());
                self.advance();
                result
            }

            other => {
                return Err(format!("Expected property value, found {other:?}"));
            }
        };

        self.expect(&Token::Semicolon)?;

        Ok(Property { name, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn parse_source(source: &str) -> Result<Simulation, String> {
        let tokens = tokenize(source)?;
        parse(&tokens)
    }

    #[test]
    fn parses_simulation() {
        let simulation = parse_source(
            r#"
            simulation "PWFA_Test" {
                density = 1e17;
                description = "Baseline plasma";
            }
            "#,
        )
        .unwrap();

        assert_eq!(simulation.name, "PWFA_Test");
        assert_eq!(simulation.statements.len(), 2);

        assert_eq!(
            simulation.statements[0],
            Statement::Property(Property {
                name: "density".into(),
                value: Value::Number(1e17),
            })
        );
    }

    #[test]
    fn parses_string_output() {
        let simulation = parse_source(r#"simulation "Test" { output("Hello"); }"#).unwrap();

        assert_eq!(
            simulation.statements[0],
            Statement::Output(Value::String("Hello".into()))
        );
    }

    #[test]
    fn parses_numeric_output() {
        let simulation = parse_source(r#"simulation "Test" { output(42); }"#).unwrap();

        assert_eq!(
            simulation.statements[0],
            Statement::Output(Value::Number(42.0))
        );
    }

    #[test]
    fn rejects_empty_output() {
        assert!(parse_source(r#"simulation "Test" { output(); }"#).is_err());
    }

    #[test]
    fn rejects_output_expression_for_now() {
        assert!(parse_source(r#"simulation "Test" { output(10 + 20); }"#).is_err());
    }

    #[test]
    fn rejects_multiple_output_arguments() {
        assert!(parse_source(r#"simulation "Test" { output("A", "B"); }"#).is_err());
    }

    #[test]
    fn rejects_unknown_function() {
        assert!(parse_source(r#"simulation "Test" { mystery(42); }"#).is_err());
    }

    #[test]
    fn parses_plasma_and_beam_blocks() {
        let simulation = parse_source(
            r#"
            simulation "PWFA_Test" {
                plasma {
                    density = 1e17;
                }

                beam {
                    energy = 1.0;
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(simulation.name, "PWFA_Test");
        assert_eq!(simulation.statements.len(), 2);

        assert_eq!(
            simulation.statements[0],
            Statement::Block(Block {
                name: "plasma".into(),
                statements: vec![Statement::Property(Property {
                    name: "density".into(),
                    value: Value::Number(1e17),
                })],
            })
        );

        assert_eq!(
            simulation.statements[1],
            Statement::Block(Block {
                name: "beam".into(),
                statements: vec![Statement::Property(Property {
                    name: "energy".into(),
                    value: Value::Number(1.0),
                })],
            })
        );
    }

    #[test]
    fn parses_deeply_nested_blocks() {
        let simulation = parse_source(
            r#"
            simulation "Nested" {
                plasma {
                    profile {
                        density = 1e17;
                    }
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(simulation.statements.len(), 1);

        let Statement::Block(plasma) = &simulation.statements[0] else {
            panic!("Expected plasma block");
        };

        let Statement::Block(profile) = &plasma.statements[0] else {
            panic!("Expected profile block");
        };

        assert_eq!(profile.name, "profile");
        assert_eq!(profile.statements.len(), 1);
    }

    #[test]
    fn rejects_missing_semicolon() {
        assert!(parse_source(r#"simulation "Test" { density = 1e17 }"#).is_err());
    }

    #[test]
    fn rejects_unclosed_block() {
        assert!(parse_source(r#"simulation "Test" { density = 1e17;"#).is_err());
    }

    #[test]
    fn rejects_trailing_tokens() {
        assert!(parse_source(r#"simulation "Test" {} extra"#).is_err());
    }

    #[test]
    fn rejects_unclosed_nested_block() {
        assert!(
            parse_source(
                r#"
                simulation "Test" {
                    plasma {
                        density = 1e17;
                }
                "#
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_invalid_statement() {
        assert!(parse_source(r#"simulation "Test" { plasma density = 1e17; }"#).is_err());
    }
}
