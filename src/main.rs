#[allow(unused_imports)]

mod my_command;
mod decode_command;

use crate::my_command::CMDS;
use crate::decode_command::decode;
use my_command::MyCommand;
use std::io::{self, Write};

fn main() {
    loop {
        //print!("$ ");
        //io::stdout().flush().unwrap();

        eprint!("$ ");
        io::stderr().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // read the command
        let my_command = MyCommand::new(&input.trim());

        // parse the command
        let _res = decode(my_command);

        io::stdout().flush().unwrap();
    }
}