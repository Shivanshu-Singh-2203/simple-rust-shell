#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
        while true {
                let mut command = String::new();
                print!("$ ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut command).unwrap();
                println!("{}: command not found", command.trim());
        }
}
