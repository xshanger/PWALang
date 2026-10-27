use crate::ast::{Property, Simulation, Value};
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

        let mut properties = Vec::new();

        while self.current() != Some(&Token::RightBrace) {
            if self.current().is_none() {
                return Err("Unclosed simulation block".into());
            }

            properties.push(self.parse_property()?);
        }

        self.expect(&Token::RightBrace)?;

        if self.current().is_some() {
            return Err("Unexpected tokens after simulation".into());
        }

        Ok(Simulation { name, properties })
    }

    fn parse_property(&mut self) -> Result<Property, String> {
        let name = match self.current() {
            Some(Token::Identifier(value)) => {
                let name = value.clone();
                self.advance();
                name
            }
            other => {
                return Err(format!("Expected property name, found {other:?}"));
            }
        };

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

    #[test]
    fn parses_simulation() {
        let source = r#"
            simulation "PWFA_Test" {
                density = 1e17;
                description = "Baseline plasma";
            }
        "#;

        let tokens = tokenize(source).unwrap();
        let simulation = parse(&tokens).unwrap();

        assert_eq!(simulation.name, "PWFA_Test");
        assert_eq!(simulation.properties.len(), 2);
        assert_eq!(simulation.properties[0].name, "density");
        assert_eq!(simulation.properties[0].value, Value::Number(1e17));
    }

    #[test]
    fn rejects_missing_semicolon() {
        let tokens = tokenize(r#"simulation "Test" { density = 1e17 }"#).unwrap();

        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn rejects_unclosed_block() {
        let tokens = tokenize(r#"simulation "Test" { density = 1e17;"#).unwrap();

        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn rejects_trailing_tokens() {
        let tokens = tokenize(r#"simulation "Test" {} extra"#).unwrap();

        assert!(parse(&tokens).is_err());
    }
}
