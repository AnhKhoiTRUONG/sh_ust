use std::process::Command;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Word,
    Pipe,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

pub fn parse(cmd_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let mut cmd_iter = cmd_string.chars();

    while let Some(token) = parse_char(&mut cmd_iter, &mut buffer) {
        //borrow error
        tokens.push(token);
        buffer = String::new();
    }
    println!("{:?}", tokens);
    tokens
}

pub fn parse_char(chars: &mut Chars, buffer: &mut String) -> Option<Token> {
    if let Some(c) = chars.next() {
        println!("{c}");
        match c {
            '\n' => {
                if !buffer.is_empty() {
                    Some(Token {
                        token_type: TokenType::Word,
                        value: buffer.clone(),
                    })
                } else {
                    None
                }
            }

            ' ' => {
                if !buffer.is_empty() {
                    Some(Token {
                        token_type: TokenType::Word,
                        value: buffer.clone(),
                    })
                } else {
                    parse_char(chars, buffer)
                }
            }

            '|' => Some(Token {
                token_type: TokenType::Pipe,
                value: String::from(""),
            }),
            _ => {
                buffer.push(c);
                parse_char(chars, buffer)
            }
        }
    } else {
        None
    }
}

pub fn parse_cmd(cmd_string: String) -> Option<Command> {
    let vec_char = cmd_string.split_whitespace().collect::<Vec<&str>>();

    let mut iter = vec_char.iter();

    let cmd_name = iter.next()?; // this return None if empty
    let mut command = Command::new(cmd_name);

    for arguments in iter {
        command.arg(arguments);
    }
    Some(command)
}
