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

        let mut command = input.split_whitespace();

        let head = command.next();
        let tail = command.collect::<Vec<&str>>().join(" ");

        match head {
            Some("exit 0") => break,
            Some("echo ") => println!("{}", tail),
            Some("type ") => {
                match tail.trim() {
                    "exit 0" | "exit" | "echo" | "type" => println!("{} is a shell builtin", tail),
                    _ => println!("{}: not found", tail),
                }
            }
            _ => println!("{}: command not found", input.trim())
        }
    }
}
