#[allow(unused_imports)]
use std::io::{self, Write};

pub fn run() {

    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();
        let commandInput: Vec<&str> = input.splitn(2, ' ').collect();


        // println!("{}", commandInput[1]);
        let cmd = commandInput.get(0);
        // println!("{}", cmd.unwrap());
        match cmd {
            Some(&"exit") => continue,
            Some(&"echo") => {
                print!("hhh");
                if commandInput.len() > 1 {
                    println!("{}", commandInput[1]);
                }
            }
            Some(&"type") => {
                if commandInput.len() > 1 {
                    println!("{}", commandInput[1]);
                }
            }
            Some(cmd) => {
                println!("{}: command not found", cmd);
            }
            None => println!("Enter a command"),
            // _ => {}
        }
    }
}
