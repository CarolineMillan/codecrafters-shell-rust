#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::{env::{current_dir, set_current_dir, var}, io::{self, Write}, path::Path, process::Command};
use std::process::exit;
use std::fmt::Write as OtherWrite;

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
const CMDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

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

// this was from chatgpt and it seems to work, make sure you understand it
fn parse_input(input: &str) -> Option<(Option<String>, Vec<String>)> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let mut tokens = Vec::new();
    let mut current = String::new();

    //let mut in_single_quotes = false;
    
    enum QuoteState {
        NoQuote,
        Single,
        Double,
    }
    use QuoteState::*;
    
    let mut state = NoQuote;
    let mut chars = input.chars().peekable();

    // loop through every character in the string -- this feels like a brute force method, is there a better way?
    while let Some(c) = chars.next() {
        match state {
            NoQuote => {
                // If we see a whitespace and we're not inside quotes,
                // then the current token (if not empty) is complete.
                if c.is_whitespace() {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    // Consume all whitespace.
                    /*
                    while let Some(&ws) = chars.peek() {
                        if ws.is_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    */
                }
                // Toggle quoting state on encountering a single quote.
                else if c == '\''{
                    // Consume the quote.
                    //chars.next();
                    // Toggle the flag.
                    state = Single;
                }
                else if c == '"' {
                    state = Double;
                }
                else if c == '\\' {
                    // OR it means we keep the next char and drop the backslash
                    if let Some(&next_char) = chars.peek() {
                        current.push(next_char);
                        chars.next();
                    }
                }
                // Any other character is added to the current token.
                else {
                    current.push(c);
                }
            }
            Single => {
                if c == '\'' {
                    // End of single-quoted string.
                    state = NoQuote;
                } else {
                    // In single quotes, all characters are literal.
                    current.push(c);
                }
            }
            Double => {
                if c == '"' {
                    // End of double-quoted string.
                    state = NoQuote;
                } else if c == '\\' {
                    // In double quotes, backslash escapes specific characters.
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '\\' || next_char == '$' || next_char == '"' || next_char == '\n' {
                            // Consume the escaped character.
                            current.push(next_char);
                            chars.next();
                        } else {
                            // Backslash remains if it doesn't escape one of the above.
                            current.push(c);
                        }
                    } else { 
                        current.push(c);
                    }
                } else {
                    current.push(c);
                }
            }
        }
    }
    // Push any token left in current.
    if !current.is_empty() {
        tokens.push(current);
    }

    // If no tokens were collected, return None.
    if tokens.is_empty() {
        return None;
    }

    // Use the first token as the head and the remaining as the arguments.
    let head = tokens.remove(0);
    Some((Some(head), tokens))
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
        "cat" => {
            
            let out = Command::new("cat")
                            .args(my_command.tail)
                            .output()
                            .expect("failed to execute process");
            io::stdout().write_all(&out.stdout).unwrap();
        
        },
        _ => {
            if let Some(_path) = find_executable_in_path::<String>(&my_head) {
                let output = Command::new(&my_head)
                                    .args(my_command.tail)
                                    .output()
                                    .expect("failed to execute file");
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


pub fn output_string(tail: &Vec<String>, output: &str) {
    /*
    PLAN
    - take the output string and tail
    - decide whether to just use println!, or write to file (ie redirect output)
    
    tail[1] will be 1> or >
    tail[2] will be the filepath

     */

    let mut new_tail = tail.clone();

    if (new_tail.len() > 1) && ((new_tail[1] == ">") || (new_tail[1] == "1>")) {
        // redirect output to tail[2]
        //let mut file = File::create(tail[2])?;
        //file.write_all(header.as_bytes())?;
        write!(&mut new_tail[2], "{}", output).unwrap();
    } else {
        println!("{}", output)
    }
    
}


/* 
fn process_echo_argument(argument: &str) -> String {
    if is_surrounded_by_quotes(argument) {
        remove_surrounding_quotes(argument)
    } else {
        argument.split_whitespace().collect::<Vec<&str>>().join(" ")
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
    //rest = rest.replace("\'", "");

    let result = rest
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();//.join(" ")

    
    Some((Some(head.to_string()),result))
}
*/