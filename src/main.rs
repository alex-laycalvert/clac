use std::io;

use clac::evaluate;

fn main() {
    println!("clac - it's a thing\n");

    let mut input = String::new();

    loop {
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if bytes_read == 0 {
            break;
        }
        if let Some(r) = evaluate(input.clone()) {
            println!("{r}");
        }
        input.clear();
    }
}
