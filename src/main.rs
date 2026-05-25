use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn parse_cmd(cmd_string: String) -> Option<Command> {
    let vec_char = cmd_string.split_whitespace().collect::<Vec<&str>>();

    let mut iter = vec_char.iter();

    let cmd_name = iter.next()?; // this return None if empty
    let mut command = Command::new(cmd_name);

    for arguments in iter {
        command.arg(arguments);
    }
    Some(command)
}

fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

fn main() {
    let mut input = String::new();
    // let mut command: Command;

    print!("{} >>> ", current_dir().display());
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let Some(mut cmd) = parse_cmd(input) {
                // println!("{:?}", cmd);
                let output = cmd.status().expect("can't execute command");
            }
        }
        Err(error) => print!("error: {error}"),
    }
}
