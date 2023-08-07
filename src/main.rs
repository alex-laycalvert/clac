use linked_list::{Cursor, LinkedList};
use std::io::{self, Write};
use std::iter::FromIterator;

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(f32),
    Add,
    Subtract,
    Divide,
    Multiply,
    Exponent,
    LeftParen,
    RightParen,
}

impl Token {
    pub fn to_string(&self) -> String {
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
    }
}

#[derive(Debug)]
enum Expr {
    Literal(f32),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Literal(l) => format!("{l}"),
            Self::Grouping(expr) => format!("(group {})", expr.to_string()),
            Self::Binary(left, op, right) => format!(
                "({} {} {})",
                op.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Self::Unary(op, expr) => format!("({} {})", op.to_string(), expr.to_string()),
        }
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        match execute(line) {
            Ok(result) => println!("= {result}"),
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn execute(line: String) -> Result<f32, String> {
    let tokens = tokenize(line)?;
    let expression = parse(tokens)?;
    evaluate(expression)
}

fn evaluate(expression: Expr) -> Result<f32, String> {
    match expression {
        Expr::Literal(val) => Ok(val),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(op, expr) => match op {
            Token::Subtract => Ok(-evaluate(*expr)?),
            _ => Err(String::from("Failed to evaluate")),
        },
        Expr::Binary(left, op, right) => match op {
            Token::Add => Ok(evaluate(*left)? + evaluate(*right)?),
            Token::Subtract => Ok(evaluate(*left)? - evaluate(*right)?),
            Token::Multiply => Ok(evaluate(*left)? * evaluate(*right)?),
            Token::Divide => Ok(evaluate(*left)? / evaluate(*right)?),
            Token::Exponent => Ok(evaluate(*left)?.powf(evaluate(*right)?)),
            _ => Err(String::from("Failed to evaluate")),
        },
    }
}

fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = LinkedList::from_iter(tokens.iter());
    let mut cursor = tokens.cursor();
    parse_expression(&mut cursor)
}

fn parse_expression(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    parse_term(tokens)
}

fn parse_term(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    let mut expr = parse_factor(tokens)?;
    while match_tokens(tokens, &[Token::Subtract, Token::Add]) {
        let op = **tokens.peek_prev().unwrap();
        let right = parse_factor(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }
    Ok(expr)
}

fn parse_factor(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    let mut expr = parse_base(tokens)?;
    while match_tokens(tokens, &[Token::Divide, Token::Multiply]) {
        let op = **tokens.peek_prev().unwrap();
        let right = parse_base(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }
    Ok(expr)
}

fn parse_base(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    let mut expr = parse_unary(tokens)?;
    while match_tokens(tokens, &[Token::Exponent]) {
        let op = **tokens.peek_prev().unwrap();
        let right = parse_unary(tokens)?;
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
    }
    Ok(expr)
}

fn parse_unary(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    if match_tokens(tokens, &[Token::Subtract]) {
        let right = parse_unary(tokens)?;
        return Ok(Expr::Unary(**tokens.peek_prev().unwrap(), Box::new(right)));
    }
    parse_primary(tokens)
}

fn parse_primary(tokens: &mut Cursor<'_, &Token>) -> Result<Expr, String> {
    if match_tokens(tokens, &[Token::Number(0.0)]) {
        if let Token::Number(val) = **tokens.peek_prev().unwrap() {
            return Ok(Expr::Literal(val));
        }
    }
    if match_tokens(tokens, &[Token::LeftParen]) {
        let expr = parse_expression(tokens)?;
        consume(
            tokens,
            Token::RightParen,
            String::from("Expected \")\" after expression."),
        )?;
        return Ok(Expr::Grouping(Box::new(expr)));
    }
    Err(String::from("Expected expression."))
}

fn consume(
    tokens: &mut Cursor<'_, &Token>,
    token_type: Token,
    message: String,
) -> Result<Token, String> {
    if check(tokens, token_type) {
        return Ok(**tokens.next().unwrap());
    }
    Err(message)
}

fn match_tokens(tokens: &mut Cursor<'_, &Token>, token_types: &[Token]) -> bool {
    for token_type in token_types {
        if check(tokens, *token_type) {
            tokens.next();
            return true;
        }
    }
    false
}

fn check(tokens: &mut Cursor<'_, &Token>, token_type: Token) -> bool {
    if let Some(token) = tokens.peek_next() {
        return match (*token, token_type) {
            (Token::Add, Token::Add) => true,
            (Token::Subtract, Token::Subtract) => true,
            (Token::Multiply, Token::Multiply) => true,
            (Token::Divide, Token::Divide) => true,
            (Token::Exponent, Token::Exponent) => true,
            (Token::LeftParen, Token::LeftParen) => true,
            (Token::RightParen, Token::RightParen) => true,
            (Token::Number(_), Token::Number(_)) => true,
            _ => false,
        };
    }
    false
}

fn tokenize(line: String) -> Result<Vec<Token>, String> {
    let mut line = line.chars().peekable();
    let mut tokens: Vec<Token> = vec![];
    while let Some(c) = line.next() {
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
                    while let Some(c) = line.peek() {
                        if !c.is_numeric() {
                            break;
                        }
                        literal.push(*c);
                        line.next();
                    }
                } else {
                    while let Some(c) = line.peek() {
                        if !c.is_numeric() {
                            break;
                        }
                        literal.push(*c);
                        line.next();
                    }
                    // NOTE: Allow trailing decimal
                    if let Some('.') = line.peek() {
                        literal.push('.');
                        line.next();
                        while let Some(c) = line.peek() {
                            if !c.is_numeric() {
                                break;
                            }
                            literal.push(*c);
                            line.next();
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
