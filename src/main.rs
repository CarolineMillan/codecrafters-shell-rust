#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::{env::{self, current_dir, join_paths, set_current_dir, split_paths}, io::{self, Write}, ops::Deref, path::PathBuf, process::Command, str::SplitWhitespace};
use std::process::exit;

/*

This is like the chip-8 interpreter
the input string is like an opcode (create a struct becuase it has a structure!!)
which we then parse ("decode") using a match statement
and execute usign other hlper functions

so we want a command struct
a fetch method (creates an instance of command struct using input)

a parse method (match statement)

and execute methods (called in parse match statement)

the main func will be a fde loop basically
*/

pub struct MyCommand<'a> {
    // head is the command and tail is the arguments (maybe rename the fields..)
    head: Option<&'a str>,
    tail: Vec<&'a str>,
}

// valid commands
const CMDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

impl<'a> MyCommand<'a> {
    fn new(input: &'a str) -> Self {
        let mut my_command = input.split_whitespace();

        let head = my_command.next();
        let tail = my_command.collect::<Vec<&str>>();//.join(" ");

        Self {
            head,
            tail,
        }
    }
}

fn parse(my_command: MyCommand) -> Result<(), Box<dyn std::error::Error>> {
    //println!("{}", my_command.head.unwrap());
    match my_command.head.unwrap() {
        // exit w code 0
        "exit" => exit(my_command.tail[0].parse().unwrap()),
        "echo" => println!("{}", my_command.tail.join(" ")),
        "type" => {
            for arg in my_command.tail.clone().into_iter() {
                if CMDS.contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } 
                else if let Some(path) = find_executable_in_path(arg) {
                    println!("{} is {}", arg, path.to_str().unwrap());
                    // change
                } 
                else {
                    println!("{} not found", my_command.tail.join(" "));
                }
            }
        }
        "pwd" => {
            let curr_dir = current_dir().expect("Problem getting current directory").into_os_string().into_string().expect("Error getting current directory as string.");
            println!("{}", curr_dir);
        }
        "cd" => {
            //let path = find_executable_in_path(my_command.head.clone().unwrap()).unwrap();
            let res = set_current_dir(my_command.tail[0]);//.expect(format!("cd: {}: No such file or directory", my_command.tail[0]).as_ref());
            if res.is_err() {
                println!("cd: {}: No such file or directory", my_command.tail[0]);
            }
        }
        _ => {
            if let Some(_path) = find_executable_in_path(my_command.head.clone().unwrap()) {
                let output = Command::new(my_command.head.unwrap()).args(my_command.tail).output().expect("failed to execute file");
                io::stdout().write_all(&output.stdout)?;
            }
            else {
                println!("{}: command not found", my_command.head.unwrap())
            }
        }
    }
    Ok(())
}


fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // read the command
        let my_command = MyCommand::new(&input.trim());

        // parse the command
        let _res = parse(my_command);
        
    }
}

/*

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        /*

        let mut command = input.split_whitespace();

        // I'm not sure about using this, purely because of exit 0 
        // I'm not sure if the 0 is necessary 
        let head = command.next();
        let tail = command.collect::<Vec<&str>>().join(" ");
*/
        match head.unwrap() {
            "exit" => break,
            "exit 0" => break,
            "echo" => println!("{}", tail),
            "type" => {
                match tail.trim() {
                    "exit" | "exit 0" | "echo" | "type" => println!("{} is a shell builtin", tail),
                    //search for tail in path and print path, else print invalid command
                    _ => {
                        // if valid path
                        // i don't know how to test for a valid path
                        /// what's the structure?
                        /// oh no
                        /// is this some kind of search algorithm
                        /// i think it is
                        /// i don't want to learn those yet!
                        /// use .split_paths
                        /// 
                        if let Some(path) = find_executable_in_path(arg) {
                            println!("{} is {}", arg, path.to_str().unwrap());
                        }
                        else {
                        // else
                        println!("{}: not found", tail)
                        }
                    },
                }
            }
            _ => println!("{}: command not found", input.trim())
        }
    }
}
*/