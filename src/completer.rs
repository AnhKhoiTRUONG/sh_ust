use crate::builtin::{BUILTIN_CMDS, list_all_executable};
use rustyline::Context;
use rustyline::completion::Completer;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};
use std::process::Command;
use std::{collections::HashMap, fs};

#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct MyCompleter {
    pub completer_reg: HashMap<String, String>,
}

fn find_files(line: &str) -> Vec<String> {
    let mut many_paths = line.split('/').collect::<Vec<&str>>();

    let paths;
    let file_find;

    if many_paths.len() > 1 {
        file_find = many_paths.pop().unwrap();
        paths = fs::read_dir(many_paths.join("/")).unwrap();
    } else {
        many_paths.pop();
        paths = fs::read_dir("./").unwrap();
        file_find = line;
    }
    let mut candidates = Vec::new();
    for path in paths {
        let mut p = path.unwrap().file_name().to_string_lossy().to_string();

        let full_path = if !many_paths.is_empty() {
            format!("{}/{}", many_paths.join("/"), p)
        } else {
            p.clone()
        };
        let check_file = std::path::Path::new(full_path.as_str());

        if p.starts_with(file_find) {
            if check_file.is_dir() {
                p.push('/');
            } else if check_file.is_file() {
                p.push(' ');
            }
            candidates.push(p);
        }
    }
    // println!("every candidates:{:?}", candidates);
    candidates
}

impl Completer for MyCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let line_as_string = line.to_string();
        let arg: Vec<&str> = line_as_string.split(' ').collect::<Vec<&str>>();

        let mut new_pos = 0;

        let mut candidates = Vec::new();

        //Gotta check if it has a completer register first
        match self.completer_reg.get(arg[0]) {
            Some(file_completer) => {
                // println!("{:?}", format!("./{}", file_completer));
                let cmd_name = arg[0];
                let need_to_complete = arg[arg.len() - 1];
                let prev_word_need_to_complete = if !arg.is_empty() {
                    arg[arg.len() - 2]
                } else {
                    ""
                };
                let candidate_stdout = Command::new(file_completer)
                    .arg(cmd_name)
                    .arg(need_to_complete)
                    .arg(prev_word_need_to_complete)
                    .env("COMP_LINE", line)
                    .env("COMP_POINT", line.len().to_string())
                    .output()
                    .unwrap();
                let candidate_stdout_string = String::from_utf8_lossy(&candidate_stdout.stdout);
                candidates = candidate_stdout_string
                    .split("\n")
                    .filter(|s| !s.is_empty())
                    .map(|s| format!("{} ", s))
                    .collect::<Vec<String>>();
                new_pos = pos - need_to_complete.chars().count();
            }
            None => {
                //if only found 1 word then its a command (builtin or executable) else its an argument
                if arg.len() > 1 {
                    let file_need_to_find = arg.last().unwrap();
                    candidates = find_files(file_need_to_find);
                    new_pos = pos - file_need_to_find.split("/").last().unwrap().chars().count();
                } else {
                    //we gonna find builtin first
                    for built_in in BUILTIN_CMDS {
                        if built_in.starts_with(line) {
                            let mut return_builtin = built_in.to_string().to_owned();
                            return_builtin.push(' ');
                            candidates.push(return_builtin);
                        }
                    }
                    //search for executable
                    candidates.append(&mut list_all_executable(line));
                }
            }
        };

        candidates.sort();
        Ok((new_pos, candidates))
    }
}
