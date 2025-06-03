#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    // print!("$ ");
    // io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let input = input.trim();
        match input {
            "exit" => break,
            "exit 0" => break,
            command=>{
                let commandInput: Vec<&str> = command.splitn(2, ' ').collect();

                // print!(echo[0]);
                // print!("{}", echo[0]);
                // println!("{}", echo[1]);
                let a = Vec::new();
                let cmd = a.get(3);
                match cmd {
                    Some(&"echo")=>{
                        if commandInput.len() > 1{
                            println!("{}", commandInput[1]);
                        }
                    }
                    Some(&"type")=>{
                        if commandInput.len() > 1 {
                            println!("{}", commandInput[1]);
                        }
                    }
                    Some(cmd) => {
                        println!("{}: command not found", cmd);
                    },
                    None=>println!("Enter a command"),
                    // _ => {}
                }
                // if echo.len() > 1 && echo[0] == "echo" {
                //     println!("{}", echo[1]);
                // }
                // else {
                    println!("{}: command not found", command)
                // }
            },
        }
    }
}
