use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::error::ReadlineError;
//use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Helper, Context, Result as RLResult};
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::{env, fs};
use std::path::Path;

// Our custom completer
pub struct MyCompleter {
    last_prefix: Option<String>,
    tab_count: usize,
}

impl MyCompleter {
    pub fn new() -> Self {
        MyCompleter {
            last_prefix: None,
            tab_count: 0,
        }
    }

    fn find_executables(prefix: &str) -> Vec<String> {
        let mut matches = Vec::new();
        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                            if file_name.starts_with(prefix) && is_executable(&path) {
                                matches.push(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        matches
    }
}

fn is_executable(path: &Path) -> bool {
    path.is_file() && path.metadata().map(|m| m.permissions().readonly()).unwrap_or(false) == false
}

// Implement the Completer trait.
impl Completer for MyCompleter {
    type Candidate = MyCandidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>
    ) -> RLResult<(usize, Vec<MyCandidate>)> {
        let builtins = vec!["echo", "exit"];
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let word = &line[start..pos];

        let mut candidates = Vec::new();
        for &cmd in &builtins {
            if cmd.starts_with(word) {
                // Append a trailing space.
                candidates.push(MyCandidate(format!("{} ", cmd)));
            }
        }
        Ok((start, candidates))
    }
}

// Candidate implementation for completions.
#[derive(Debug)]
pub struct MyCandidate(String);

impl Candidate for MyCandidate {
    fn display(&self) -> &str {
        &self.0
    }
    fn replacement(&self) -> &str {
        &self.0
    }
}

// Provide a dummy implementation for Hinter.
impl Hinter for MyCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for MyCompleter {
    // Other methods...
    fn highlight<'a, 'b>(&'a self, line: &'b str, _pos: usize) -> Cow<'b, str> {
        Cow::Borrowed(line)
    }

    fn highlight_hint<'a, 'b>(&'a self, hint: &'b str) -> Cow<'b, str> {
        Cow::Borrowed(hint)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        false
    }
}

// Provide a dummy implementation for Validator.
impl Validator for MyCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> RLResult<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

// Finally, implement the Helper trait, which requires all of the above.
impl Helper for MyCompleter {}


pub struct MyHelper {
    last_prefix: RefCell<Option<String>>, // Wrap in RefCell
    tab_count: RefCell<u8>, // Wrap in RefCell
}

impl MyHelper {
    pub fn new() -> Self {
        MyHelper {
            last_prefix: RefCell::new(None),
            tab_count: RefCell::new(0),
        }
    }
        // This function returns external executables that start with the given prefix.
        fn find_executables(prefix: &str) -> Vec<String> {
            let mut matches = Vec::new();
            if let Some(paths) = env::var_os("PATH") {
                for dir in env::split_paths(&paths) {
                    if let Ok(entries) = fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                                if file_name.starts_with(prefix) && is_executable(&path) {
                                    matches.push(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
            matches
        }
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let prefix = &line[..pos];
        
        // Get built-in commands
        let builtins = vec!["echo", "exit"];
        let mut candidates: Vec<String> = builtins
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();

        // Get external executable matches
        let mut ext_matches = Self::find_executables(prefix);
        candidates.append(&mut ext_matches);

        // Remove duplicates and sort alphabetically
        candidates.sort();
        candidates.dedup();

        // Debug prints (optional)
        // println!("Prefix: {}", prefix);
        // println!("Candidates: {:?}", candidates);

        if candidates.is_empty() {
            return Ok((pos, vec![]));
        }

        if candidates.len() == 1 {
            let candidate = candidates[0].clone();
            return Ok((pos - prefix.len(), vec![Pair {
                display: candidate.clone(),
                replacement: candidate + " ", // Append a space after completion
            }]));
        }

        // For multiple candidates, use tab_count logic
        {
            let mut last_prefix = self.last_prefix.borrow_mut();
            let mut tab_count = self.tab_count.borrow_mut();

            if last_prefix.as_deref() == Some(prefix) {
                if *tab_count == 1 {
                    // Sort candidates alphabetically for display
                    let display_candidates = candidates.join("  ");
                    println!("\n{}", display_candidates);
                    // Reprint prompt with current input (without an extra trailing space)
                    print!("$ {}", prefix.trim_end());
                    std::io::stdout().flush().unwrap();
                    *tab_count = 0; // Reset after printing suggestions
                    return Ok((pos, vec![]));
                }
                *tab_count += 1;
            } else {
                *last_prefix = Some(prefix.to_string());
                *tab_count = 1;
                print!("\x07"); // Bell on first TAB
                std::io::stdout().flush().unwrap();
            }
        }

        Ok((pos, vec![]))
    }
    
}
/*
fn is_executable(path: &Path) -> bool {
    path.is_file() && path.metadata().map(|m| m.permissions().readonly() == false).unwrap_or(false)
}
*/
impl Helper for MyHelper {}
impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}
impl Highlighter for MyHelper {}
impl Validator for MyHelper {}