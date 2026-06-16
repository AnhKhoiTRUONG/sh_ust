pub mod builtin;
pub mod completer;
pub mod lexer;
use crate::builtin::{cd_cmd, pwd_cmd, run_cmd};
use crate::completer::MyCompleter;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, Editor, history::DefaultHistory};

// fn current_dir() -> PathBuf {
//     std::env::current_dir().unwrap()
// }

// fn treat_cmd(simple_cmd: SimpleCommand) {}

fn main() {
    // let mut input: String; // let mut command: Command;
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl: Editor<MyCompleter, DefaultHistory> = Editor::with_config(config).unwrap();

    // let mut rl: Editor<MyCompleter, DefaultHistory> = Editor::new().unwrap();
    rl.set_helper(Some(MyCompleter));

    loop {
        // input = String::new();
        // print!("{} $ ", current_dir().display());
        // print!("$ ");
        // io::stdout().flush().unwrap();

        // match io::stdin().read_line(&mut input) {
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
                        _ => run_cmd(tokens),
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
