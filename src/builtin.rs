use nix::unistd::AccessFlags;
use nix::unistd::access;
use std::fs;
use std::path::PathBuf;

static BUILTIN_CMDS: [&str; 3] = ["echo", "exit", "type"];
const PATH_SEPARATED: char = ':';

//need to add redirection for this but for instant no need
pub fn echo_cmd(args: Vec<String>) {
    print!("{}", args[1]);
    for i in 2..args.len() {
        print!(" {}", args[i]);
    }
    println!("");
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

