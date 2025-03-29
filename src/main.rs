#[allow(unused_imports)]

mod input;
mod decode_command;
mod completer;


use completer::MyHelper;
use input::{MyCommand, CMDS};

use crate::decode_command::decode;
use crate::completer::MyCompleter;
//use std::io::{self, Write};
use rustyline::Editor;
use rustyline::history::{DefaultHistory, FileHistory};


fn main() {
    // Create an editor with our custom completer.
    //let mut rl = Editor::<MyCompleter, DefaultHistory>::new().unwrap();

    //rl.set_completer(Some(MyCompleter));

    let mut rl = Editor::<MyHelper, FileHistory>::new().unwrap();
    rl.set_helper(Some(MyHelper::new()));

    loop {
        // This prints the prompt and handles autocompletion automatically.
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                // Optionally add the line to history.
                let _res = rl.add_history_entry(line.as_str());

                // Process the line with your existing logic:
                let my_command = MyCommand::new(&line.trim());
                let _ = decode(my_command);

                // (Any necessary flushing or other I/O can be done here)
            }
            Err(_) => break, // e.g. Ctrl-D exits the shell
        }
    }
}


/*
fn main() {
    loop {
        //print!("$ ");
        //io::stdout().flush().unwrap();

        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // read the command
        let my_command = MyCommand::new(&input.trim());

        // parse the command
        let _res = decode(my_command);

        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}
*/