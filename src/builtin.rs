use nix::unistd::AccessFlags;
use nix::unistd::access;
use std::env::home_dir;
use std::env::set_current_dir;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::lexer::RedirectionType;
use crate::lexer::SimpleCommand;

pub static BUILTIN_CMDS: [&str; 6] = ["echo", "exit", "type", "pwd", "cd", "complete"];
const PATH_SEPARATED: char = ':';

pub fn execute_builtins(cmd: SimpleCommand) {
    if cmd.redirection.is_empty() {}
}

//need to add redirection for this but for instant no need
pub fn echo_cmd(cmd: SimpleCommand) {
    let args = cmd.command;

    if cmd.redirection.is_empty() {
        if args.len() > 1 {
            print!("{}", args[1]);
            for i in 2..args.len() {
                print!(" {}", args[i]);
            }
        }
        println!();
    } else {
        for redir in cmd.redirection {
            match redir.redir_type {
                RedirectionType::Out => {
                    let output_file_path = Path::new(redir.file.as_str());
                    let file = match File::create(output_file_path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("shell: {}: {}", redir.file, e);
                            panic!()
                        }
                    };
                    let mut writer = BufWriter::new(file);
                    if args.len() > 1 {
                        if let Err(e) = write!(writer, "{}", args[1]) {
                            eprintln!("shell: write error: {}", e);
                        }

                        for i in 2..args.len() {
                            if let Err(e) = write!(writer, " {}", args[i]) {
                                eprintln!("shell: write error: {}", e);
                            }
                        }
                        if let Err(e) = writeln!(writer) {
                            eprintln!("shell: write error: {}", e);
                        }
                    }
                }
                RedirectionType::OutStderr => {
                    //stdin like normal
                    print!("{}", args[1]);
                    for i in 2..args.len() {
                        print!(" {}", args[i]);
                    }
                    println!();

                    //what error can echo give?????
                    let output_file_path = Path::new(redir.file.as_str());
                    match File::create(output_file_path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("shell: {}: {}", redir.file, e);
                            panic!()
                        }
                    };
                }

                RedirectionType::Append => {
                    let mut file = match File::options()
                        .append(true)
                        .create(true)
                        .open(redir.file.as_str())
                    {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("shell: {}: {}", redir.file, e);
                            panic!()
                        }
                    };

                    if args.len() > 1 {
                        if let Err(e) = write!(&mut file, "{}", args[1]) {
                            eprintln!("shell: write error: {}", e);
                        }

                        for i in 2..args.len() {
                            if let Err(e) = write!(&mut file, " {}", args[i]) {
                                eprintln!("shell: write error: {}", e);
                            }
                        }
                        if let Err(e) = writeln!(&mut file) {
                            eprintln!("shell: write error: {}", e);
                        }
                    }
                }

                RedirectionType::AppendStderr => {
                    //stdin like normal
                    print!("{}", args[1]);
                    for i in 2..args.len() {
                        print!(" {}", args[i]);
                    }
                    println!();

                    //what error can echo give?????
                    match File::create(redir.file.as_str()) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("shell: {}: {}", redir.file, e);
                            panic!()
                        }
                    };
                }
                _ => todo!(),
            }
        }
    }
}

pub fn pwd_cmd() {
    if let Ok(path) = std::env::current_dir() {
        println!("{}", path.display());
    } else {
        panic!("Cannot retrieve current directory");
    }
}

pub fn cd_cmd(args: Vec<String>) {
    if args.len() > 2 {
        println!("cd: too many arguments");
        return;
    }

    let path = if args[1] == "~" {
        home_dir().unwrap()
    } else {
        PathBuf::from(args[1].as_str())
    };
    change_dir(&path);
}

fn change_dir(path: &PathBuf) {
    if set_current_dir(path).is_err() {
        eprintln!("cd: {}: No such file or directory", path.display());
    }
}

fn is_builtin(cmd: &str) -> bool {
    BUILTIN_CMDS.contains(&cmd)
}

fn find_executable(cmd: &str) -> Option<PathBuf> {
    // eprintln!("DEBUG PATH: {:?}", std::env::var("PATH"));
    // eprintln!("DEBUG cmd name bytes: {:?}", cmd.as_bytes());
    match std::env::var("PATH") {
        Ok(path) => {
            let all_path = path.split(PATH_SEPARATED).collect::<Vec<&str>>();
            for p in all_path {
                let path_find: PathBuf = [p, cmd].iter().collect();
                // println!("path: {}", path_find.display());
                if access(&path_find, AccessFlags::X_OK).is_ok() {
                    return Some(path_find);
                }
            }
        }
        Err(e) => eprintln!("$PATH not set: {}", e),
    }
    None
}

pub fn list_all_executable(line: &str) -> Vec<String> {
    // eprintln!("DEBUG PATH: {:?}", std::env::var("PATH"));
    // eprintln!("DEBUG cmd name bytes: {:?}", cmd.as_bytes());
    let mut candidates = Vec::new();
    match std::env::var("PATH") {
        Ok(whole_path) => {
            let all_path = whole_path.split(PATH_SEPARATED).collect::<Vec<&str>>();
            for paths in all_path {
                let path_dir = std::path::Path::new(paths);
                if path_dir.is_dir() {
                    let exec_path = fs::read_dir(paths).unwrap();
                    for executable in exec_path {
                        let mut cmd = executable
                            .unwrap()
                            .file_name()
                            .to_string_lossy()
                            .to_string();
                        if cmd.starts_with(line) {
                            cmd.push(' ');
                            candidates.push(cmd);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("$PATH not set: {}", e),
    }
    candidates
}

pub fn type_cmd(args: Vec<String>) {
    for i in 1..args.len() {
        let cmd = args[i].as_str();
        if is_builtin(cmd) {
            println!("{} is a shell builtin", cmd);
        } else if let Some(path) = find_executable(cmd) {
            println!("{} is {}", cmd, path.display());
        } else {
            println!("{}: not found", cmd);
        }
    }
}

//check if the command exist then passed the arguments in
pub fn run_cmd(simple_command: SimpleCommand) {
    let args = simple_command.command;
    let cmd_name = args[0].clone();
    if find_executable(cmd_name.as_str()).is_some() {
        let mut cmd = Command::new(cmd_name);
        for arg in args.iter().skip(1) {
            cmd.arg(arg);
        }

        // Print the output
        if simple_command.redirection.is_empty() {
            cmd.status().unwrap();
        } else {
            for redir in simple_command.redirection {
                match redir.redir_type {
                    RedirectionType::Out => {
                        let output_file_path = Path::new(redir.file.as_str());
                        let file = match File::create(output_file_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("shell: {}: {}", redir.file, e);
                                panic!()
                            }
                        };
                        cmd.stdout(file).status().unwrap();
                    }
                    RedirectionType::OutStderr => {
                        let output_file_path = Path::new(redir.file.as_str());
                        let file = match File::create(output_file_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("shell: {}: {}", redir.file, e);
                                panic!()
                            }
                        };
                        cmd.stderr(file).status().unwrap();
                    }

                    RedirectionType::Append => {
                        let file = match File::options()
                            .append(true)
                            .create(true)
                            .open(redir.file.as_str())
                        {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("shell: {}: {}", redir.file, e);
                                panic!()
                            }
                        };

                        cmd.stdout(file).status().unwrap();
                    }

                    RedirectionType::AppendStderr => {
                        let file = match File::options()
                            .append(true)
                            .create(true)
                            .open(redir.file.as_str())
                        {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("shell: {}: {}", redir.file, e);
                                panic!()
                            }
                        };

                        cmd.stderr(file).status().unwrap();
                    }
                    _ => todo!(),
                }
            }
        }
    } else {
        println!("{}: command not found", cmd_name);
    }
}
