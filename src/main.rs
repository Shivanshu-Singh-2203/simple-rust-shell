#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
        loop {
                print!("$ ");
                io::stdout().flush().unwrap();
                let mut commands = String::new();
                io::stdin().read_line(&mut commands).unwrap();

                match commands.trim() {
                        "exit" => {
                                break;
                        }
                        a => {
                                if a.len() > 0 {
                                        println!("{a}: command not found");
                                } else {
                                        break;
                                }
                        }
                }
        }
}
