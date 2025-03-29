use rustyline::completion::{Candidate, Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Helper, Context, Result as RLResult};
use std::borrow::Cow;

// Our custom completer
pub struct MyCompleter;

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
    /*
    fn highlight_prompt<'a, 'b>(&'a self, prompt: &'b str, _default: bool) -> Cow<'b, str>
    where
        Self: 'a,
    {
        Cow::Borrowed(prompt)
    }
    */

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


/*
// Provide a dummy implementation for Highlighter.
impl Highlighter for MyCompleter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }
    fn highlight_prompt<'l>(&self, prompt: &'l str, default: bool) -> Cow<'l, str>
    where
        Self: 'l,
    {
        Cow::Borrowed(prompt)
    } 
    fn highlight_hint<'l>(&self, hint: &'l str) -> Cow<'l, str> {
        Cow::Borrowed(hint)
    }
    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        false
    }
    
    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str, // FIXME should be Completer::Candidate
        completion: rustyline::CompletionType,
    ) -> Cow<'c, str> {
        let _ = completion;
        Cow::Borrowed(candidate)
    }
}
*/

// Provide a dummy implementation for Validator.
impl Validator for MyCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> RLResult<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

// Finally, implement the Helper trait, which requires all of the above.
impl Helper for MyCompleter {}


pub struct MyHelper;

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        // Your completion logic here
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