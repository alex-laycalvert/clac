mod expr;
mod token;

use expr::Expr;
use std::io::{self, Write};
use token::Token;

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
    let tokens = Token::tokenize(line)?;
    let expression = Expr::from(tokens)?;
    expression.evaluate()
}
