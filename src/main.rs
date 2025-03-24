#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::{env::{self, current_dir, join_paths, set_current_dir, split_paths, var}, io::{self, Write}, ops::Deref, path::{Path, PathBuf}, process::Command, str::SplitWhitespace};
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
const CMDS: [&str; 6] = ["exit", "echo", "type", "pwd", "cd", "cat"];

impl<'a> MyCommand<'a> {
    fn new(input: &'a str) -> Self {
        //let mut my_command = input.trim().split_whitespace();

        //let head = my_command.next();
        let (head, tail) = parse_input(input).expect("Error parsing input");//.collect::<Vec<&str>>());//.join(" ");

        Self {
            head,
            tail,
        }
    }
}

fn decode(my_command: MyCommand) -> Result<(), Box<dyn std::error::Error>> {
    //println!("{}", my_command.head.unwrap());
    match my_command.head.unwrap() {
        // exit w code 0
        "exit" => exit(my_command.tail[0].parse().unwrap()),
        "echo" => echo(my_command.tail),
        "type" => {
            for arg in my_command.tail.clone().into_iter() {
                if CMDS.contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } 
                else if let Some(path) = find_executable_in_path(arg) {
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
        "cd" => change_directory(my_command.tail[0]),
        "cat" => {}
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

fn echo(args: Vec<&str>) {
    /*
    let mut my_args = Vec::<&str>::new();
    for arg in args.clone() {
        /*
        let mut chars = arg.chars();
        let mut my_arg = arg;
        if (chars.nth(0) == chars.nth_back(0)) &&  (chars.nth(0) == Some('\'')) {
            chars.next();
            chars.next_back();
            my_arg = chars.as_str();
        }
        */
        let my_arg: &str = arg.trim_matches(|c| c == '\"' || c == '\'').as_ref();
        //let my_arg: &str = temp.to_string().as_ref();
        my_args.push(my_arg);
    }
    */
    println!("{}", args.join(" "))
}

fn parse_input<'a>(input: &str) -> Option<(Option<&str>, Vec<&str>)> { //} (Option<&str>, Option<Vec<&str>>) {
    let input = input.trim();
    let (cmd, rest) = input.split_once(" ").unwrap_or((input, ""));
    let mut result = vec![];//cmd];//.to_string()];
    let mut rest = rest.trim();
    while !rest.is_empty() {
        match rest.chars().next().unwrap() {
            '\'' => {
                let (arg, r) = rest[1..].split_once('\'')?;
                result.push(arg);//.to_string());
                rest = r;
            }
            ' ' => {
                rest = rest.trim_start();
            }
            _c => {
                let (arg, r) = rest.split_once(' ').unwrap_or((rest, ""));
                result.push(arg);//.to_string());
                rest = r;
            }
        }
        rest = rest.trim();
    }
    Some((Some(cmd), result))
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