use crate::builtins;
#[allow(unused_imports)]
use std::io::{self, Write};

pub fn run() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();
        let commandInput: Vec<&str> = input.splitn(2, ' ').collect();
        let cmd = commandInput.get(0);
        match cmd {
            Some(&"exit") => continue,
            Some(&"echo") => {
                    builtins::echo_cmd(commandInput.get(1).map(|v| *v));
            }
            Some(&"type") => {
                builtins::type_cmd(commandInput.get(1).map(|v| *v));
            }
            Some(cmd) => {
                println!("{}: command not found", cmd);
            }
            None => println!("Enter a command"),
            // _ => {}
        }
    }
}
