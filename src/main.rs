mod expr;
mod token;

use expr::Expr;
use std::io::{self, Write};
use token::Token;

fn main() {
    prompt();
}

fn prompt() {
    let mut ans = None;
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        match execute(line, ans) {
            Ok(result) => {
                ans = Some(result);
                println!("= {result}");
            }
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn execute(source: String, ans: Option<f32>) -> Result<f32, String> {
    let tokens = Token::tokenize(source)?;
    let expression = Expr::from(tokens, ans)?;
    expression.evaluate()
}
