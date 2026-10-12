use anyhow::{Context, Ok, anyhow};
use strum_macros::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
        pub name: String,
        pub path: Option<String>,
        pub args: Vec<String>,
}

#[derive(Debug, PartialEq, Display)]
pub enum Cmd {
        Echo(Command),
        Exit(u8),
        Type(Command),
        Unknown(Command),
        Exec(Command),
}

impl Command {
        pub fn new(name: &str, path: Option<String>, args: Vec<String>) -> Self {
                Self {
                        name: name.to_owned(),
                        path: path,
                        args,
                }
        }

        pub fn run(&self) -> anyhow::Result<()> {
                let program = &self.name;
                let mut child = std::process::Command::new(program)
                        .args(&self.args)
                        .spawn()?;
                child.wait()?;

                Ok(())
        }
}
