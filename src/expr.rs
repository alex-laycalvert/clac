use crate::token::Token;

use linked_list::{Cursor, LinkedList};
use std::iter::FromIterator;

#[derive(Debug)]
pub enum Expr {
    Literal(f32),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{l}"),
            Self::Grouping(expr) => write!(f, "(group {expr})"),
            Self::Binary(left, op, right) => {
                write!(f, "({op} {left} {right})")
            }
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
        }
    }
}

impl Expr {
    pub fn evaluate(&self) -> Result<f32, String> {
        match self {
            Expr::Literal(val) => Ok(*val),
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Unary(op, expr) => match op {
                Token::Subtract => Ok(-expr.evaluate()?),
                t => Err(format!("Failed to evaluate: \"{t}\"")),
            },
            Expr::Binary(left, op, right) => match op {
                Token::Add => Ok(left.evaluate()? + right.evaluate()?),
                Token::Subtract => Ok(left.evaluate()? - right.evaluate()?),
                Token::Multiply => Ok(left.evaluate()? * right.evaluate()?),
                Token::Divide => Ok(left.evaluate()? / right.evaluate()?),
                Token::Exponent => Ok(left.evaluate()?.powf(right.evaluate()?)),
                _ => Err(String::from("Failed to evaluate")),
            },
        }
    }

    pub fn from(tokens: Vec<Token>, ans: Option<f32>) -> Result<Self, String> {
        let mut tokens = LinkedList::from_iter(tokens.iter());
        let mut cursor = tokens.cursor();
        Self::expression(&mut cursor, ans)
    }

    fn expression(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        Self::term(tokens, ans)
    }

    fn term(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        let mut expr = Self::factor(tokens, ans)?;
        while Self::match_tokens(tokens, &[Token::Subtract, Token::Add]) {
            let op = tokens.peek_prev().unwrap().clone();
            let right = Self::factor(tokens, ans)?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        let mut expr = Self::base(tokens, ans)?;
        while Self::match_tokens(tokens, &[Token::Divide, Token::Multiply]) {
            let op = tokens.peek_prev().unwrap().clone();
            let right = Self::base(tokens, ans)?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn base(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        let mut expr = Self::unary(tokens, ans)?;
        while Self::match_tokens(tokens, &[Token::Exponent]) {
            let op = tokens.peek_prev().unwrap().clone();
            let right = Self::unary(tokens, ans)?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        if Self::match_tokens(tokens, &[Token::Subtract]) {
            let op = tokens.peek_prev().unwrap().clone();
            let right = Self::unary(tokens, ans)?;
            return Ok(Expr::Unary(op, Box::new(right)));
        }
        Self::primary(tokens, ans)
    }

    fn primary(tokens: &mut Cursor<'_, &Token>, ans: Option<f32>) -> Result<Expr, String> {
        if Self::match_tokens(tokens, &[Token::Identifier(String::new())]) {
            if let Token::Identifier(iden) = tokens.peek_prev().unwrap() {
                if iden == "ans" {
                    return Ok(Expr::Literal(ans.unwrap_or_default()));
                }
                return Err(format!("Unknown identifier \"{iden}\""));
            }
        }
        if Self::match_tokens(tokens, &[Token::Number(0.0)]) {
            if let Token::Number(val) = tokens.peek_prev().unwrap() {
                return Ok(Expr::Literal(*val));
            }
        }
        if Self::match_tokens(tokens, &[Token::LeftParen]) {
            let expr = Self::expression(tokens, ans)?;
            Self::consume(
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
        if Self::check(tokens, token_type) {
            return Ok(tokens.next().unwrap().clone());
        }
        Err(message)
    }

    fn match_tokens(tokens: &mut Cursor<'_, &Token>, token_types: &[Token]) -> bool {
        for token_type in token_types {
            if Self::check(tokens, token_type.clone()) {
                tokens.next();
                return true;
            }
        }
        false
    }

    fn check(tokens: &mut Cursor<'_, &Token>, token_type: Token) -> bool {
        if let Some(token) = tokens.peek_next() {
            return matches!(
                (*token, token_type),
                (Token::Add, Token::Add)
                    | (Token::Subtract, Token::Subtract)
                    | (Token::Multiply, Token::Multiply)
                    | (Token::Divide, Token::Divide)
                    | (Token::Exponent, Token::Exponent)
                    | (Token::LeftParen, Token::LeftParen)
                    | (Token::RightParen, Token::RightParen)
                    | (Token::Number(_), Token::Number(_))
                    | (Token::Identifier(_), Token::Identifier(_))
            );
        }
        false
    }
}
