#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
        loop {
                print!("$ ");
                io::stdout().flush().unwrap();
                let mut commands = String::new();
                io::stdin().read_line(&mut commands).unwrap();

                let tokens: Vec<&str> = commands.trim().split_whitespace().collect();

                if tokens.len() > 0 {
                        match tokens[0] {
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
