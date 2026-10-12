#[allow(unused_imports)]
use std::io::{self, Write};

fn check_type_command(tokens: Vec<&str>) {
        for i in 1..tokens.len() {
                match tokens[i] {
                        "echo" | "exit" | "type" => {
                                println!("{} is a shell builtin", tokens[i]);
                        }
                        _ => {
                                println!("{}: not found", tokens[i]);
                        }
                }
        }
}

fn main() {
        loop {
                print!("$ ");
                io::stdout().flush().unwrap();
                let mut commands = String::new();
                io::stdin().read_line(&mut commands).unwrap();

                let tokens: Vec<&str> = commands.trim().split_whitespace().collect();

                if tokens.len() > 0 {
                        match tokens[0] {
                                "type" => {
                                        check_type_command(tokens);
                                }
                                "exit" => {
                                        break;
                                }
                                "echo" => {
                                        let left = tokens[1..].join(" ");
                                        println!("{left}");
                                }
                                _ => {
                                        println!("{}: command not found", tokens[0]);
                                }
                        }
                } else {
                        continue;
                }
        }
}
