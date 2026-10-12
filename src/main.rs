#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn check_type_command(tokens: Vec<&str>) {
        for i in 1..tokens.len() {
                let command = tokens[i];

                match command {
                        "echo" | "exit" | "type" => {
                                println!("{} is a shell builtin", command);
                        }
                        _ => {
                                let mut found = false;

                                if let Ok(path) = env::var("PATH") {
                                        for dir in env::split_paths(&path) {
                                                let full_path = dir.join(command);

                                                if let Ok(metadata) = std::fs::metadata(&full_path)
                                                {
                                                        if metadata.is_file()
                                                                && metadata.permissions().mode()
                                                                        & 0o111
                                                                        != 0
                                                        {
                                                                println!(
                                                                        "{} is {}",
                                                                        command,
                                                                        full_path.display()
                                                                );
                                                                found = true;
                                                                break;
                                                        }
                                                }
                                        }
                                }

                                if !found {
                                        println!("{}: not found", command);
                                }
                        }
                }
        }
}

fn main() {
        loop {
                print!("$ ");
                io::stdout().flush().unwrap();

                let mut commands = String::new();

                if io::stdin().read_line(&mut commands).unwrap() == 0 {
                        break;
                }

                let tokens: Vec<&str> = commands.split_whitespace().collect();

                if tokens.is_empty() {
                        continue;
                }

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
        }
}
