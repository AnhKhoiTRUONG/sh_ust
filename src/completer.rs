use crate::builtin::{BUILTIN_CMDS, list_all_executable};
use rustyline::Context;
use rustyline::completion::Completer;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct MyCompleter;

impl Completer for MyCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        //we gonna find builtin first
        let mut candidates = Vec::new();
        for built_in in BUILTIN_CMDS {
            if built_in.starts_with(line) {
                // candidates.push(built_in.to_string());
                let mut return_builtin = built_in.to_string().to_owned();
                return_builtin.push(' ');
                candidates.push(return_builtin);
                // return Ok((0, vec![return_builtin]));
            }
        }

        //search for executable
        candidates.append(&mut list_all_executable(line));

        candidates.sort();
        Ok((0, candidates))
    }
}
