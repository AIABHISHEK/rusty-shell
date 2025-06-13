use crate::builtins;
use crate::util;
#[allow(unused_imports)]
use std::io::{self, Write};

pub fn run() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut output_ = String::new();
        // let mut err_ = String::new();
        let mut redirect = false;
        let mut file: Option<String> = None;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let v = util::parse_command_line(input, &mut redirect, &mut file);
        // for val in v {
        //     println!("{val}");
        // }

        // let commandInput: Vec<&str> = input.splitn(2, ' ').collect();
        let c = &v[0];
        let cmd = c.as_str();
        let args = &v[1..].iter().map(|s| s.clone()).collect();
        match cmd {
            "exit" => {
                if v.len() > 1 && v.get(1).unwrap() != "0" {
                    println!("invalid argument for exit")
                } else {
                    return;
                }
            }
            "echo" => {
                // builtins::echo_cmd(commandInput.get(1).map(|v| *v));
                builtins::echo_cmd(args, &mut output_);
            }
            "type" => {
                builtins::type_cmd(args);
            }
            "pwd" => {
                builtins::pwd_cmd(&mut output_);
            }
            "cd" => {
                builtins::cd_cmd(args, &mut output_);
            }
            command => {
                // println!("{}: command not found", cmd);
                builtins::existing_command(command, args, &mut output_);
            } // _ => {}
        }
        match file {
            Some(f) => {
                let trimmed = output_.trim_end_matches('\n').to_string();
                builtins::write_to_file(format!("{}\n", trimmed), f);
            },
            None => {
                if !output_.is_empty() {
                    println!("{output_}")
                }
            },
        }
    }
}
