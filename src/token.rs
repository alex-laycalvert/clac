#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f32),
    Add,
    Subtract,
    Divide,
    Multiply,
    Exponent,
    LeftParen,
    RightParen,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(val) => val.to_string(),
                Self::Add => String::from("+"),
                Self::Subtract => String::from("-"),
                Self::Multiply => String::from("*"),
                Self::Divide => String::from("/"),
                Self::Exponent => String::from("^"),
                Self::LeftParen => String::from("("),
                Self::RightParen => String::from(")"),
            }
        )
    }
}

impl Token {
    pub fn tokenize(source: String) -> Result<Vec<Self>, String> {
        let mut source = source.chars().peekable();
        let mut tokens: Vec<Token> = vec![];
        while let Some(c) = source.next() {
            match c {
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Subtract),
                '/' => tokens.push(Token::Divide),
                '*' => tokens.push(Token::Multiply),
                '^' => tokens.push(Token::Exponent),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                ' ' | '\r' | '\t' => (),
                '\n' => break,
                c => {
                    if !c.is_numeric() && c != '.' {
                        return Err(format!("Failed to parse \"{c}\""));
                    }
                    let mut literal = String::new();
                    literal.push(c);
                    // NOTE: Allow leading decimal
                    if c == '.' {
                        while let Some(c) = source.peek() {
                            if !c.is_numeric() {
                                break;
                            }
                            literal.push(*c);
                            source.next();
                        }
                    } else {
                        while let Some(c) = source.peek() {
                            if !c.is_numeric() {
                                break;
                            }
                            literal.push(*c);
                            source.next();
                        }
                        // NOTE: Allow trailing decimal
                        if let Some('.') = source.peek() {
                            literal.push('.');
                            source.next();
                            while let Some(c) = source.peek() {
                                if !c.is_numeric() {
                                    break;
                                }
                                literal.push(*c);
                                source.next();
                            }
                        }
                    }
                    match literal.parse() {
                        Ok(val) => tokens.push(Token::Number(val)),
                        Err(e) => return Err(format!("Failed to parse \"{literal}\" ({e})")),
                    }
                }
            }
        }
        Ok(tokens)
    }
}
