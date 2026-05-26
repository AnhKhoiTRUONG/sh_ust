pub mod lexer;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::lexer::parse_cmd;

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn main() {
    let mut input: String; // let mut command: Command;
    // loop {
    input = String::new();
    print!("{} >>> ", current_dir().display());
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some(mut cmd) = parse_cmd(input.clone()) {
                // println!("{:?}", cmd);
                let output = cmd.status().expect("can't execute command");
            }
        }
        Err(error) => print!("error: {error}"),
    }
    // }

    lexer::parse(input);
}
