use std::{env::{set_current_dir, var}, path::Path};


pub struct MyCommand {
    // head is the command and tail is the arguments (maybe rename the fields..)
    pub head: Option<String>,
    pub tail: Vec<String>,
    pub output_location: OutputLocation,
}

#[derive(Debug)]
pub enum OutputLocation {
    Console,
    File(String),  // File path for output redirection
    AppendToFile(String),  // Append to file for '>>'
}

impl OutputLocation {
    pub fn get_filepath(&self) -> Option<&str> {
        match self {
            OutputLocation::File(ref filepath) => Some(filepath.as_str()),
            OutputLocation::AppendToFile(ref filepath) => Some(filepath.as_str()),
            _ => None,
        }
    }
}


// valid commands
pub const CMDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

impl MyCommand {
    pub fn new(input: &str) -> Self {
        let (head, tail, output_location) = parse_input(input).expect("Error parsing input");//.collect::<Vec<&str>>());//.join(" ");

        Self {
            head,
            tail,
            output_location,
        }
    }
}


// this was from chatgpt and it seems to work, make sure you understand it
fn parse_input(input: &str) -> Option<(Option<String>, Vec<String>, OutputLocation)> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    // GET TOKENS

    let mut tokens = Vec::new();
    let mut current = String::new();
    
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
                if c.is_whitespace() {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                else if c == '\''{
                    state = Single;
                }
                else if c == '"' {
                    state = Double;
                }
                else if c == '\\' {
                    if let Some(&next_char) = chars.peek() {
                        current.push(next_char);
                        chars.next();
                    }
                }
                else {
                    current.push(c);
                }
            }
            Single => {
                if c == '\'' {
                    state = NoQuote;
                } else {
                    current.push(c);
                }
            }
            Double => {
                if c == '"' {
                    state = NoQuote;
                } else if c == '\\' {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '\\' || next_char == '$' || next_char == '"' || next_char == '\n' {
                            current.push(next_char);
                            chars.next();
                        } else {
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

    // SET OUTPUT LOCATION
    let (filtered_tokens, output_location) = set_output_location(&head, tokens);

    Some((Some(head), filtered_tokens, output_location))
}


fn set_output_location(head: &str, tokens: Vec<String>) -> (Vec<String>, OutputLocation) {
    // Default output location.
    let mut output_location = OutputLocation::Console;

    // New vector to hold tokens that are not related to redirection.
    let mut filtered_tokens = Vec::new();
    let mut iter = tokens.into_iter();

    //let head = tokens[0].clone();

    // Scan tokens for redirection operators.
    while let Some(token) = iter.next() {
        if token == ">" || token == "1>" {
            // Next token is the file path.
            if let Some(filepath) = iter.next() {
                // check it's a valid filepath
                if valid(&filepath) {
                    output_location = OutputLocation::File(filepath);
                }
                else {
                    println!("{}: {}: No such file or directory", head, filepath)
                }
            }
        } else if token == ">>" {
            if let Some(filepath) = iter.next() {
                if valid(&filepath) {
                    output_location = OutputLocation::AppendToFile(filepath);
                }
                else {
                    println!("{}: {}: No such file or directory", head, filepath)
                }
            }
        } else {
            filtered_tokens.push(token);
        }
    }
    (filtered_tokens, output_location)
}


fn valid(dir: &str) -> bool {
    if dir == "~" {
        // Get the home directory path and check if it exists and is a directory.
        if let Ok(home) = var("HOME") {
            let path = Path::new(&home);
            return path.exists() && path.is_dir();
        } else {
            return false;
        }
    } else {
        // For a given directory, check if it exists and is a directory.
        let path = Path::new(dir);
        return path.exists() && path.is_dir();
    }
}


/* 
fn valid(dir: &str) -> bool {

    if dir == "~" {
        let path = var("HOME").expect("Error getting home directory");
        let res = set_current_dir(path);

        if res.is_err() {
            return false;
            //println!("cd: {}: No such file or directory", dir);
        }
        else {return true;}
    }
    else {
        
        // get a handle on input path
        let path = Path::new(dir).canonicalize();
        
        if path.is_err() {
            return false;
            //println!("cd: {}: No such file or directory", dir);
        }
        else {
            let res = set_current_dir(path.unwrap());

            if res.is_err() {
                return false;
                //println!("cd: {}: No such file or directory", dir);
            
            }
            else {return true;}
        }
    }
}
*/
/*
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



use std::path::Path;

while let Some(token) = iter.next() {
    if token == ">" || token == "1>" {
        // Next token is the file path.
        if let Some(filepath) = iter.next() {
            let path = Path::new(&filepath);
            // Check if the parent directory exists.
            // If there is no parent, assume the file is in the current directory.
            let valid = if let Some(parent) = path.parent() {
                parent.exists()
            } else {
                true
            };

            if valid {
                output_location = OutputLocation::File(filepath);
            } else {
                eprintln!("Error: Directory for file path '{}' does not exist.", filepath);
                // You can decide what to do next: either set a default location,
                // skip redirection, or return an error.
            }
        }
    } else if token == ">>" {
        if let Some(filepath) = iter.next() {
            let path = Path::new(&filepath);
            let valid = if let Some(parent) = path.parent() {
                parent.exists()
            } else {
                true
            };

            if valid {
                output_location = OutputLocation::AppendToFile(filepath);
            } else {
                eprintln!("Error: Directory for file path '{}' does not exist.", filepath);
            }
        }
    } else {
        filtered_tokens.push(token);
    }
}
*/