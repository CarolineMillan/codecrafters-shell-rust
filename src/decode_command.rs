
use crate::CMDS;
use crate::my_command::OutputLocation;
use crate::MyCommand;
use pathsearch::find_executable_in_path;
use std::{env::{current_dir, set_current_dir, var}, fs::{File, OpenOptions}, io::{self, Write}, path::Path, process::Command};
use std::process::exit;

pub fn decode(my_command: MyCommand) -> Result<(), Box<dyn std::error::Error>> {
    //println!("{}", my_command.head.unwrap());
    let my_head = my_command.head.unwrap();
    match my_head.as_ref() {
        // exit w code 0
        "exit" => exit(my_command.tail[0].parse().unwrap()),
        "echo" => {
            let output = format!("{}", my_command.tail.join(" "));
            let _res = output_string(&output, &my_command.output_location);
        }
        "type" => {
            //let arg = my_command.tail.clone().into_iter().map(|x| x.as_str());
            for arg in my_command.tail.clone().into_iter() {
                if CMDS.contains(&arg.as_ref()) {
                    let output = format!("{} is a shell builtin", arg);
                    let _res = output_string(&output, &my_command.output_location);
                } 
                else if let Some(path) = find_executable_in_path(&arg) {
                    let output = format!("{} is {}", arg, path.to_str().unwrap());
                    let _res = output_string(&output, &my_command.output_location);
                } 
                else {
                    let output = format!("{} not found", my_command.tail.join(" "));
                    let _res = output_string(&output, &my_command.output_location);
                }
            }
        }
        "pwd" => {
            let curr_dir = current_dir().expect("Problem getting current directory").into_os_string().into_string().expect("Error getting current directory as string.");
            let output = format!("{}", curr_dir);
            let _res = output_string(&output, &my_command.output_location);
            //println!("{}", curr_dir);
        }
        "cd" => change_directory(&my_command.tail[0]),
        "cat" => {
            // the problem is in this section
            let out = Command::new("cat")
                            .args(my_command.tail)
                            .output()
                            .expect("failed to execute process");
            io::stdout().write_all(&out.stdout).expect("failed to write all to stdout");
        },
        _ => {
            if let Some(_path) = find_executable_in_path::<String>(&my_head) {
                let output = Command::new(&my_head)
                                    .args(&my_command.tail)
                                    .output()
                                    .expect("failed to execute file");
                                let output_str = std::str::from_utf8(&output.stdout)?;
                                output_string(output_str, &my_command.output_location)?;
            }
            else {
                let output = format!("{}: command not found", &my_head);
                let _res = output_string(&output, &my_command.output_location);
                //println!("{}: command not found", &my_head)
            }
        }
    }
    Ok(())
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


pub fn output_string(output: &str, output_location: &OutputLocation) -> Result<(), Box<dyn std::error::Error>> {
    
    match output_location {
        OutputLocation::Console => {
            println!("{}", output);
        }
        OutputLocation::File(file_path) => {
            let mut file = File::create(file_path)?;
            writeln!(file, "{}", output)?;
        }
        OutputLocation::AppendToFile(file_path) => {
            let mut file = OpenOptions::new().append(true).create(true).open(file_path)?;
            writeln!(file, "{}", output)?;
        }
    }
    Ok(())
}