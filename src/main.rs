#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::{env::{current_dir, set_current_dir, var}, io::{self, Write}, path::Path, process::Command};
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

pub struct MyCommand {
    // head is the command and tail is the arguments (maybe rename the fields..)
    head: Option<String>,
    tail: Vec<String>,
}

// valid commands
const CMDS: [&str; 6] = ["exit", "echo", "type", "pwd", "cd", "cat"];

impl MyCommand {
    fn new(input: &str) -> Self {
        //let mut my_command = input.trim().split_whitespace();

        //let head = my_command.next();
        let (head, tail) = parse_input(input).expect("Error parsing input");//.collect::<Vec<&str>>());//.join(" ");


        Self {
            head,
            tail,
        }
    }
}

fn parse_input(input: &str) -> Option<(Option<String>, Vec<String>)> { //} (Option<&str>, Option<Vec<&str>>) {
    
    // get head
    let input = input.trim();
    let (head, rest) = input.split_once(" ").unwrap_or((input, ""));
    
    //let mut result = vec![];
    let mut rest = rest.trim().to_string();

    // removed surrounding quotes, need to do this for all args
    if is_surrounded_by_quotes(&rest) {
        rest = remove_surrounding_quotes(&rest);
    }

    // Remove any inner quotes.
    rest = rest.replace("\'", "");

    let result = rest
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();//.join(" ")

    
    Some((Some(head.to_string()),result))
}

fn decode(my_command: MyCommand) -> Result<(), Box<dyn std::error::Error>> {
    //println!("{}", my_command.head.unwrap());
    let my_head = my_command.head.unwrap();
    match my_head.as_ref() {
        // exit w code 0
        "exit" => exit(my_command.tail[0].parse().unwrap()),
        "echo" => println!("{}", my_command.tail.join(" ")),
        "type" => {
            //let arg = my_command.tail.clone().into_iter().map(|x| x.as_str());
            for arg in my_command.tail.clone().into_iter() {
                if CMDS.contains(&arg.as_ref()) {
                    println!("{} is a shell builtin", arg);
                } 
                else if let Some(path) = find_executable_in_path(&arg) {
                    println!("{} is {}", arg, path.to_str().unwrap());
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
        "cd" => change_directory(&my_command.tail[0]),
        "cat" => {}
        _ => {
            if let Some(_path) = find_executable_in_path::<String>(&my_head) {
                let output = Command::new(&my_head).args(my_command.tail).output().expect("failed to execute file");
                io::stdout().write_all(&output.stdout)?;
            }
            else {
                println!("{}: command not found", &my_head)
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
        let _res = decode(my_command);
        
    }
}

fn change_directory(dir: &str) {
    if dir == "~" {
        let path = var("HOME").expect("Error getting home directory");
        let res = set_current_dir(path);

        if res.is_err() {
            println!("cd: {}: No such file or directory", dir);
        }
    }
    else {
        // get a handle on input path
        let path = Path::new(dir).canonicalize();
        
        if path.is_err() {
            println!("cd: {}: No such file or directory", dir);
        }
        else {
            let res = set_current_dir(path.unwrap());
    
            if res.is_err() {
                println!("cd: {}: No such file or directory", dir);
            }
        }
    }
}

pub fn remove_surrounding_quotes(argument: &str) -> String {
    argument.trim_matches(|c| c == '"' || c == '\'').to_string()
}

pub fn is_surrounded_by_quotes(argument: &str) -> bool {
    matches!(argument.chars().next(), Some('\'')) && matches!(argument.chars().last(), Some('\''))
        || matches!(argument.chars().next(), Some('\"'))
            && matches!(argument.chars().last(), Some('\"'))
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