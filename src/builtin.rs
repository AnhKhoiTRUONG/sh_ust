use nix::unistd::AccessFlags;
use nix::unistd::access;
use std::env;
use std::env::home_dir;
use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::Command;

static BUILTIN_CMDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];
const PATH_SEPARATED: char = ':';

//need to add redirection for this but for instant no need
pub fn echo_cmd(args: Vec<String>) {
    print!("{}", args[1]);
    for i in 2..args.len() {
        print!(" {}", args[i]);
    }
    println!("");
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

    let path: PathBuf;

    if args[1] == "~" {
        path = home_dir().unwrap();
    } else {
        path = PathBuf::from(args[1].as_str());
    }
    change_dir(&path);
}

fn change_dir(path: &PathBuf) {
    if !set_current_dir(path).is_ok() {
        println!("cd: {}: No such file or directory", path.display());
    }
}

fn is_builtin(cmd: &str) -> bool {
    BUILTIN_CMDS.contains(&cmd)
}

fn find_executable(cmd: &str) -> Option<PathBuf> {
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
pub fn run_cmd(args: Vec<String>) {
    let cmd_name = args[0].clone();
    if find_executable(cmd_name.as_str()).is_some() {
        let mut cmd = Command::new(cmd_name);
        for arg in args.iter().skip(1) {
            cmd.arg(arg);
        }
        cmd.status().unwrap();
    } else {
        println!("{}: command not found", cmd_name);
    }
}
