#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    Equals,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        match c {
            '{' => {
                tokens.push(Token::LeftBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RightBrace);
                i += 1;
            }
            '=' => {
                tokens.push(Token::Equals);
                i += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            '"' => {
                i += 1;
                let start = i;

                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }

                if i == chars.len() {
                    return Err("Unterminated string".into());
                }

                let value: String = chars[start..i].iter().collect();
                tokens.push(Token::StringLiteral(value));
                i += 1;
            }
            '0'..='9' => {
                let start = i;
                i += 1;

                while i < chars.len()
                    && (chars[i].is_ascii_digit()
                        || chars[i] == '.'
                        || chars[i] == 'e'
                        || chars[i] == 'E'
                        || ((chars[i] == '+' || chars[i] == '-')
                            && i > start
                            && (chars[i - 1] == 'e' || chars[i - 1] == 'E')))
                {
                    i += 1;
                }

                let value: String = chars[start..i].iter().collect();

                let number = value
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {value}"))?;

                tokens.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                i += 1;

                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }

                let value: String = chars[start..i].iter().collect();
                tokens.push(Token::Identifier(value));
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            _ => {
                return Err(format!("Unexpected character: {c}"));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_simulation() {
        let source = r#"simulation "Test" { density = 1e17; }"#;
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0], Token::Identifier("simulation".into()));
        assert_eq!(tokens[1], Token::StringLiteral("Test".into()));
        assert_eq!(tokens[4], Token::Equals);
        assert_eq!(tokens[5], Token::Number(1e17));
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(tokenize("@").is_err());
    }

    #[test]
    fn rejects_unterminated_strings() {
        assert!(tokenize(r#""unfinished"#).is_err());
    }

    #[test]
    fn test_arithmetic_tokens() {
        let source = "10 + 20 * (3 - 1) / 2 ^ 2";

        let tokens = tokenize(source).unwrap();

        let expected = vec![
            Token::Number(10.0),
            Token::Plus,
            Token::Number(20.0),
            Token::Star,
            Token::LeftParen,
            Token::Number(3.0),
            Token::Minus,
            Token::Number(1.0),
            Token::RightParen,
            Token::Slash,
            Token::Number(2.0),
            Token::Caret,
            Token::Number(2.0),
        ];

        assert_eq!(tokens, expected);
    }
}
