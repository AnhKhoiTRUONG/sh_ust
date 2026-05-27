pub mod builtin;
pub mod lexer;
use std::io::{self, Write};
use std::path::PathBuf;

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn main() {
    let mut input: String; // let mut command: Command;

    loop {
        input = String::new();
        print!("{} $ ", current_dir().display());
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = lexer::token_to_command(lexer::parse(input));
                // if tokens.command.is_empty() {
                //     println!("")
                let cmd = tokens.command[0].as_str();
                match cmd {
                    "exit" => break,
                    "echo" => builtin::echo_cmd(tokens.command),
                    "type" => builtin::type_cmd(tokens.command),
                    _ => println!("{}: command not found", cmd),
                }
            }
            Err(error) => print!("error: {error}"),
        }
    }

    // lexer::parse(input);
}
