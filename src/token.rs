use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f32),
    Identifier(String),
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
                Self::Identifier(i) => i.to_string(),
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
                    if c.is_numeric() || c == '.' {
                        Self::process_number(c, &mut source, &mut tokens)?;
                    } else if c.is_ascii_alphabetic() {
                        Self::process_identifier(c, &mut source, &mut tokens)?;
                    } else {
                        return Err(format!("Failed to parse \"{c}\""));
                    }
                }
            }
        }
        Ok(tokens)
    }

    fn process_number(
        c: char,
        source: &mut Peekable<Chars<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String> {
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
            Ok(val) => {
                tokens.push(Token::Number(val));
                Ok(())
            }
            Err(e) => Err(format!("Failed to parse \"{literal}\" ({e})")),
        }
    }

    fn process_identifier(
        c: char,
        source: &mut Peekable<Chars<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String> {
        let mut literal = String::new();
        literal.push(c);
        while let Some(c) = source.peek() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            literal.push(*c);
            source.next();
        }
        tokens.push(Token::Identifier(literal));
        Ok(())
    }
}
