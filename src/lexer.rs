use std::iter::Peekable;
use std::process::Command;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Word,
    Pipe,
    RedirIn,
    RedirOutStderr,
    RedirOut,
    Append,
    AppendStderr,
    Heredoc,
}

#[derive(Debug)]
pub enum RedirectionType {
    In,
    Out,
    OutStderr,
    Heredoc,
    Append,
    AppendStderr,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[derive(Debug)]
pub struct Redirection {
    pub redir_type: RedirectionType,
    pub file: String,
}

pub struct SimpleCommand {
    pub command: Vec<String>,
    pub redirection: Vec<Redirection>,
}

pub fn parse(cmd_string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();
    let cmd_iter = cmd_string.chars();
    let mut chars = cmd_iter.peekable();

    while let Some(token) = parse_char(&mut chars, &mut buffer) {
        //borrow error
        tokens.push(token);
        buffer = String::new();
    }
    // println!("{:?}", tokens);
    tokens
}

fn parse_char(chars_peekable: &mut Peekable<Chars<'_>>, buffer: &mut String) -> Option<Token> {
    // let mut chars_peekable = chars.peekable();
    if let Some(c) = chars_peekable.peek() {
        // println!("in parche char:{c}");
        match c {
            '\n' => {
                // println!("hi");
                chars_peekable.next();
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
                chars_peekable.next();
                if !buffer.is_empty() {
                    Some(Token {
                        token_type: TokenType::Word,
                        value: buffer.clone(),
                    })
                } else {
                    parse_char(chars_peekable, buffer)
                }
            }

            '|' => {
                chars_peekable.next();
                Some(Token {
                    token_type: TokenType::Pipe,
                    value: String::from(""),
                })
            }

            '2' => {
                chars_peekable.next();
                if chars_peekable.next_if(|&x| x == '>').is_some() {
                    if chars_peekable.next_if(|&x| x == '>').is_some() {
                        return Some(Token {
                            token_type: TokenType::AppendStderr,
                            value: String::from(""),
                        });
                    }
                    Some(Token {
                        token_type: TokenType::RedirOutStderr,
                        value: String::from(""),
                    })
                } else {
                    buffer.push('2');
                    // println!("what is char: {:?}", chars);
                    parse_char(chars_peekable, buffer)
                }
            }

            '>' => {
                chars_peekable.next();
                if chars_peekable.next_if(|&x| x == '>').is_some() {
                    return Some(Token {
                        token_type: TokenType::Append,
                        value: String::from(""),
                    });
                }
                Some(Token {
                    token_type: TokenType::RedirOut,
                    value: String::from(""),
                })
            }

            '<' => {
                chars_peekable.next();
                //next_if consume chars cause peekable is the wrapper of chars
                if chars_peekable.next_if(|&x| x == '<').is_some() {
                    return Some(Token {
                        token_type: TokenType::Heredoc,
                        value: String::from(""),
                    });
                }
                Some(Token {
                    token_type: TokenType::RedirIn,
                    value: String::from(""),
                })
            }

            '\\' => {
                chars_peekable.next();
                if let Some(escaped_car) = chars_peekable.next() {
                    buffer.push(escaped_car);
                    parse_char(chars_peekable, buffer)
                } else {
                    todo!()
                }
            }

            '"' => {
                chars_peekable.next();
                parse_quote(chars_peekable, buffer)
            }

            '\'' => {
                chars_peekable.next();
                parse_single_quote(chars_peekable, buffer)
            }

            _ => {
                let car = chars_peekable.next().unwrap();
                buffer.push(car);
                // chars.next();
                parse_char(chars_peekable, buffer)
            }
        }
    } else if !buffer.is_empty() {
        Some(Token {
            token_type: TokenType::Word,
            value: buffer.clone(),
        })
    } else {
        None
    }
}
//
fn parse_quote(chars_peekable: &mut Peekable<Chars<'_>>, buffer: &mut String) -> Option<Token> {
    // let mut chars_peekable = chars.peekable();
    if let Some(c) = chars_peekable.peek() {
        // println!("{c}");
        if *c == '"' {
            chars_peekable.next();
            parse_char(chars_peekable, buffer)
        } else if *c == '\\' {
            let mut char_add = chars_peekable.next().unwrap();
            if let Some(car) = chars_peekable.peek() {
                if *car == ' ' {
                    buffer.push(char_add);
                    char_add = chars_peekable.next().unwrap();
                    buffer.push(char_add);
                    parse_quote(chars_peekable, buffer)
                } else {
                    char_add = chars_peekable.next().unwrap();
                    buffer.push(char_add);
                    parse_quote(chars_peekable, buffer)
                }
            } else {
                panic!("doesnt found \"");
            }
        } else {
            let car = chars_peekable.next().unwrap();
            buffer.push(car);
            parse_quote(chars_peekable, buffer)
        }
    } else {
        panic!("doesnt found \"");
    }
}

fn parse_single_quote(
    chars_peekable: &mut Peekable<Chars<'_>>,
    buffer: &mut String,
) -> Option<Token> {
    // let mut chars_peekable = chars.peekable();
    if let Some(c) = chars_peekable.peek() {
        // println!("in parse single quote: {c}");
        if *c == '\'' {
            chars_peekable.next();
            parse_char(chars_peekable, buffer)
        } else {
            let car = chars_peekable.next().unwrap();
            buffer.push(car);
            parse_single_quote(chars_peekable, buffer)
        }
    } else {
        panic!("doesnt found \'");
    }
}

pub fn token_to_command(tokens: Vec<Token>) -> SimpleCommand {
    let mut cmd = SimpleCommand {
        command: Vec::new(),
        redirection: Vec::new(),
    };

    let mut tokens_iter = tokens.iter();

    //use next to be able to consume sometime 2 tokens at 1 time
    while let Some(token) = tokens_iter.next() {
        match token.token_type {
            TokenType::Word => cmd.command.push(token.value.clone()),
            TokenType::Pipe => todo!(),
            TokenType::RedirIn => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::In,
                    file: file.value.clone(),
                });
            }

            TokenType::RedirOut => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::Out,
                    file: file.value.clone(),
                });
            }

            TokenType::Append => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::Append,
                    file: file.value.clone(),
                });
            }

            TokenType::Heredoc => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::Heredoc,
                    file: file.value.clone(),
                });
            }

            TokenType::RedirOutStderr => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::OutStderr,
                    file: file.value.clone(),
                });
            }

            TokenType::AppendStderr => {
                let file = tokens_iter.next().unwrap();
                cmd.redirection.push(Redirection {
                    redir_type: RedirectionType::AppendStderr,
                    file: file.value.clone(),
                });
            }
        }
    }

    cmd
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
