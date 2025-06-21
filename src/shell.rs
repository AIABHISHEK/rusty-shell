use crate::builtins;
use crate::completer::TrieCompleter;
use crate::trie::Trie;
use crate::util;
use rustyline::{history::DefaultHistory, CompletionType, Config, Editor};
#[allow(unused_imports)]
// use std::io::{self, Write};
use std::io;
use std::io::{stdin, stdout, Write};

pub enum RedirectType {
    None,
    StdOutToFile,
    StdErrToFile,
    AppendStdOutToFile,
    AppendStdErrToFile,
}

pub fn run() {
    let mut trie = Trie::new();
    for cmd in ["echo", "exit", "pwd", "cd", "type"] {
        trie.insert(cmd);
    }
    let existing_cmd = util::get_executable();
    for cmd in existing_cmd {
        // let s = cmd.clone().as_str();
        trie.insert(cmd.as_str());
    }
    let completer = TrieCompleter { trie };
    let config = Config::builder()
        .completion_type(CompletionType::List)
         // Enables double-Tab listing
        .build();
    // let mut rl = Editor::with_config(config);
    let mut rl = Editor::<TrieCompleter, DefaultHistory>::with_config(config).unwrap(); // allows helper setup
    rl.set_helper(Some(completer));
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let rs = rl.readline("$ ");
        // stdin.read_line(&mut input).unwrap();
        match rs {
            Ok(r) => {
                let mut output_: Vec<String> = Vec::new();
                let mut err_ = String::new();
                // let mut redirect = false;
                let mut file: Option<String> = None;
                let mut redirect: RedirectType = RedirectType::None;
                // let input = input.trim();
                let input = r;
                if input.is_empty() {
                    continue;
                }
                let mut is_pipe = false;
                let v = util::parse_command_line(input.as_str(), &mut redirect, &mut file, &mut is_pipe);
                // print!("this is iped or not {}", is_pipe);
                let c = match is_pipe {
                    true=> "|",
                    false=> &v[0],
                };
                // println!("this is {}", c);
                let cmd = c;
                let args = &v[1..].iter().map(|s| s.clone()).collect();
                match cmd {
                    "|" => {
                        builtins::handle_pipe(&v, &mut output_, &mut err_, &mut redirect);
                    }
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
                        builtins::existing_command(
                            command,
                            args,
                            &mut output_,
                            &mut err_,
                            &mut redirect,
                        );
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
            Err(e) => {}
        }
    }
}
