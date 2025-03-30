use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::error::ReadlineError;
//use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Helper, Context, Result as RLResult};
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::{env, fs};
use std::path::Path;

// Our custom completer
pub struct MyCompleter {
}

impl MyCompleter {
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
    // Compute the longest common prefix from a vector of strings.
    fn longest_common_prefix(strings: &[String]) -> String {
        if strings.is_empty() {
            return String::new();
        }
        let mut prefix = strings[0].clone();
        for s in strings.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    break;
                }
            }
        }
        prefix
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

        // Built-in commands.
        let builtins = vec!["echo", "exit"];
        // Start by collecting builtins that match the prefix.
        let mut candidates: Vec<String> = builtins
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();

        // Collect external executables.
        let mut ext_matches = Self::find_executables(prefix);
        candidates.append(&mut ext_matches);

        // Remove duplicates and sort alphabetically.
        candidates.sort();
        candidates.dedup();

        // If there are no candidates, return empty.
        if candidates.is_empty() {
            return Ok((pos, vec![]));
        }

        // Compute longest common prefix among candidates.
        let lcp = Self::longest_common_prefix(&candidates);

        // If the LCP is longer than what the user has typed, then complete to that.
        if lcp.len() > prefix.len() {
            // If the LCP exactly matches one candidate, we can add a trailing space.
            let replacement = if candidates.len() == 1 { //|| lcp == candidates[0] {
                format!("{} ", lcp)
            } else {
                lcp.clone()
            };
            return Ok((pos - prefix.len(), vec![Pair {
                display: replacement.clone(),
                replacement,
            }]));
        }

        // Otherwise, if multiple candidates remain and lcp equals prefix, use tab-count logic.
        {
            let mut last_prefix = self.last_prefix.borrow_mut();
            let mut tab_count = self.tab_count.borrow_mut();

            if last_prefix.as_deref() == Some(prefix) {
                if *tab_count == 1 {
                    // On second consecutive TAB press: print all candidates.
                    println!("\n{}", candidates.join("  "));
                    print!("$ {}", prefix.trim_end());
                    std::io::stdout().flush().unwrap();
                    *tab_count = 0; // Reset tab count.
                    return Ok((pos, vec![]));
                }
                *tab_count += 1;
            } else {
                *last_prefix = Some(prefix.to_string());
                *tab_count = 1;
                print!("\x07"); // Ring bell on first TAB.
                std::io::stdout().flush().unwrap();
            }
        }

        // Otherwise, return no replacement, keeping the current input.
        Ok((pos, vec![]))
    }
    
}

impl Helper for MyHelper {}
impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        None
    }
}
impl Highlighter for MyHelper {}
impl Validator for MyHelper {}