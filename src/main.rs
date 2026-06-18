pub mod builtin;
pub mod completer;
pub mod lexer;
use crate::builtin::{cd_cmd, complete_cmd, pwd_cmd, run_cmd};
use crate::completer::MyCompleter;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, Editor, history::DefaultHistory};
use std::collections::HashMap;
use std::process::Child;

// fn current_dir() -> PathBuf {
//     std::env::current_dir().unwrap()
// }

// fn treat_cmd(simple_cmd: SimpleCommand) {}

fn main() {
    //registers for complete builtin
    // let mut input: String; // let mut command: Command;
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl: Editor<MyCompleter, DefaultHistory> = Editor::with_config(config).unwrap();

    // let mut rl: Editor<MyCompleter, DefaultHistory> = Editor::new().unwrap();
    let my_completer = MyCompleter {
        completer_reg: HashMap::new(),
    };
    rl.set_helper(Some(my_completer));

    let mut jobs: Vec<Child> = Vec::new();

    loop {
        //check the jobs
        // println!("{:?}", jobs);
        jobs.retain_mut(|job| match job.try_wait() {
            Ok(Some(_)) => false,
            Ok(None) => true,
            Err(e) => {
                println!("error attempting to wait: {e}");
                false
            }
        });

        match rl.readline("$ ") {
            Ok(input) => {
                rl.add_history_entry(&input).unwrap();
                // println!("{input}");
                let tokens = lexer::token_to_command(lexer::parse(input));
                // if tokens.command.is_empty() {
                //     println!("")
                // println!("{:?}\n{:?}", tokens.command, tokens.redirection);
                if !tokens.command.is_empty() {
                    let cmd = tokens.command[0].as_str();
                    // println!("hehe");
                    match cmd {
                        "exit" => break,
                        "echo" => builtin::echo_cmd(tokens),
                        "type" => builtin::type_cmd(tokens.command),
                        "pwd" => pwd_cmd(),
                        "cd" => cd_cmd(tokens.command),
                        "complete" => {
                            if let Some(completer_helper) = rl.helper_mut() {
                                complete_cmd(tokens.command, &mut completer_helper.completer_reg);
                            }
                        }
                        "jobs" => {}
                        _ => run_cmd(tokens, &mut jobs),
                    }
                }
            }
            Err(ReadlineError::Interrupted) => break, // Ctrl+C
            Err(ReadlineError::Eof) => break,         // Ctrl+D
            Err(e) => eprintln!("error: {e}"),
        }
    }

    // lexer::parse(input);
}
