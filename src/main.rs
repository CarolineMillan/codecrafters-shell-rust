#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {break}
        if input[0..3] == *"echo" {
            let len = input.len();
            let arg = &input[4..len];
            println!("{}", arg)
        }
        else {println!("{}: command not found", input.trim())}
    }
}
