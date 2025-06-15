use crate::builtins;
use crate::util;
#[allow(unused_imports)]
use std::io::{self, Write};

pub enum RedirectType {
    None,
    StdOutToFile,
    StdErrToFile,
    AppendStdOutToFile,
    AppendStdErrToFile,
}

pub fn run() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut output_: Vec<String> = Vec::new();
        let mut err_ = String::new();
        // let mut redirect = false;
        let mut file: Option<String> = None;
        let mut redirect: RedirectType = RedirectType::None;
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
                builtins::existing_command(command, args, &mut output_, &mut err_, &mut redirect);
            } // _ => {}
        }
        // match file {
        //     Some(f) => {
        //         // if redirect {
        //         // println!("inside {}", output_.len());
        //         if !output_.is_empty() {
        //             let st = output_.join("");
        //             // println!("test  {}", st);
        //             let trimmed = st.trim_end_matches('\n').to_string();
        //             if !trimmed.is_empty() {
        //                 builtins::write_to_file(trimmed, f);
        //             }
        //         }
        //     }
        //     None => {
        //         let st = output_.join("");
        //         let trimmed = st.trim_end_matches('\n').to_string();
        //         if !trimmed.is_empty() {
        //             println!("{}", trimmed)
        //         }
        //     }
        // }

        match redirect {
            RedirectType::None => {
                let st = output_.join("");
                let trimmed = st.trim_end_matches('\n').to_string();
                if !trimmed.is_empty() {
                    println!("{}", trimmed)
                }
                if !err_.trim_end_matches('\n').is_empty() {
                    let trimmed = err_.trim_end_matches('\n').to_string();
                    println!("{trimmed}")
                }
            }
            RedirectType::StdErrToFile => {
                let st = output_.join("");
                let trimmed = st.trim_end_matches('\n').to_string();
                if !trimmed.is_empty() {
                    println!("{}", trimmed)
                }
                match file {
                    Some(f) => {
                        let trimmed = err_.trim_end_matches('\n').to_string();
                        // println!("{trimmed}");
                        // if !trimmed.is_empty() {
                        builtins::write_to_file(trimmed, f, false);
                        // }
                    }
                    _ => {}
                }
            }
            RedirectType::StdOutToFile => {
                if !err_.trim_end_matches('\n').is_empty() {
                    let trimmed = err_.trim_end_matches('\n').to_string();
                    println!("{trimmed}")
                }
                match file {
                    Some(f) => {
                        // if redirect {
                        // println!("inside {}", output_.len());
                        if !output_.is_empty() {
                            let st = output_.join("");
                            // println!("test  {}", st);
                            let trimmed = st.trim_end_matches('\n').to_string();
                            if !trimmed.is_empty() {
                                builtins::write_to_file(trimmed, f, false);
                            }
                        }
                    }
                    None => {
                        let st = output_.join("");
                        let trimmed = st.trim_end_matches('\n').to_string();
                        if !trimmed.is_empty() {
                            println!("{}", trimmed)
                        }
                    }
                }
            }
            RedirectType::AppendStdOutToFile => {
                if !err_.trim_end_matches('\n').is_empty() {
                    let trimmed = err_.trim_end_matches('\n').to_string();
                    println!("{trimmed}")
                }
                match file {
                    Some(f) => {
                        // if !output_.is_empty() {
                            let st = output_.join("");
                            // println!("test  {}", st);
                            let trimmed = st.trim_end_matches('\n').to_string();
                            // if !trimmed.is_empty() {
                                builtins::write_to_file(trimmed, f, true);
                            // }
                        // }
                    }
                    None => {
                        let st = output_.join("");
                        let trimmed = st.trim_end_matches('\n').to_string();
                        if !trimmed.is_empty() {
                            println!("{}", trimmed)
                        }
                    }
                }
            }
            RedirectType::AppendStdErrToFile => {
                let st = output_.join("");
                let trimmed = st.trim_end_matches('\n').to_string();
                if !trimmed.is_empty() {
                    println!("{}", trimmed)
                }
                match file {
                    Some(f) => {
                        let trimmed = err_.trim_end_matches('\n').to_string();
                        // println!("{trimmed}");
                        // if !trimmed.is_empty() {
                        builtins::write_to_file(trimmed, f, true);
                        // }
                    }
                    _ => {}
                }
            }
        }
        // if !err_.trim_end_matches('\n').is_empty() {
        //     let trimmed = err_.trim_end_matches('\n').to_string();
        //     println!("{trimmed}")
        // }
    }
}
