use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::path;
use std::path::Path;
use std::process;
use std::process::Output;

pub fn echo_cmd(args: &Vec<String>, output_: &mut String, redirect: &bool) {
    if !*redirect {
        println!("{}", args.join(" "));
    }
    *output_ = args.join(" ");
}

pub fn pwd_cmd(output_: &mut String) {
    if let Ok(dir) = env::current_dir() {
        println!("{}", dir.display());
    } else {
        print!("failed to execute pwd command");
    }
}

pub fn cd_cmd(args: &Vec<String>, output_: &mut String) {
    match args.get(0).map(|c| c.as_str()) {
        Some("~") => tilde_cmd(),
        Some(dir) => match env::set_current_dir(dir.trim()) {
            Err(result) => println!("cd: {}: No such file or directory", dir.trim()),
            Ok(r) => {}
        },
        None => {}
    }
}

fn tilde_cmd() {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("/"));
    let _ = env::set_current_dir(home);
}

pub fn existing_command(command: &str, args: &Vec<String>, output_: &mut String, redirect: &mut bool) {
    // check if have > or 1> in last second index then output should be
    // let exist = write_to_file_arg_exist(args);
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let full_path = path::Path::new(dir).join(command);
            let mut cmd_args: &Vec<String> = args;
            let mut sliced_args: Vec<String> = Vec::new();
            // if exist {
            //     sliced_args = args[0..(args.len() - 2)].to_vec();
            //     cmd_args = &sliced_args;
            // }
            if full_path.exists() {
                let output = process::Command::new(command)
                    .args(cmd_args)
                    .output()
                    // .spawn()
                    .expect("command did not executed");
                // let err = &output.stderr;
                // stdout().write_all(&output.stdout);
                // if exist {
                    if output.status.success() {
                    let out = &output.stdout;

                    let s = String::from_utf8_lossy(out.as_slice());
                    let s = s.to_string();
                    *output_ = s;
                    *redirect = true;
                }
                else {
                    let s = String::from_utf8_lossy(&output.stderr.as_slice());
                    let s = s.to_string();
                    
                    // println!("{}", s);
                    *output_ = s;
                    *redirect = false;
                }
                //     let file = args[args.len() - 1].clone();
                //     write_to_file(s, file);
                // }
                // print!("output {}", String::from_utf8_lossy(out.as_slice()));
                // Optionally print stderr
                // eprint!("{}", String::from_utf8_lossy(stderr));
                return;
            }
        }
    }
    println!("{}: not found", command);
    // }
    // }
    // _ => {}
    // }
}

pub fn type_cmd(args: &Vec<String>) {
    match args.get(0) {
        Some(text) => {
            let v: Vec<&str> = text.split_ascii_whitespace().collect();
            if v.len() > 1 {
                println!("Too  many arguments");
            } else {
                match v[0] {
                    "exit" | "echo" | "type" | "pwd" | "cd" => {
                        println!("{} is a shell builtin", v[0]);
                        return;
                    }
                    _ => {}
                }
                if let Ok(path_var) = env::var("PATH") {
                    for dir in path_var.split(':') {
                        let full_path = path::Path::new(dir).join(v[0]);
                        // println!("this is full path: {:?}", full_path);
                        // println!("this is: {dir}");
                        if full_path.exists() && full_path.is_file() {
                            // execute command

                            println!("{} is {}", v[0], full_path.display());
                            return;
                        }
                    }
                }
                println!("{}: not found", v[0])
            }
        }
        _ => {}
    }
}

// fn write_to_file_exist(args: &Vec<String>) -> bool {
//     let mut exist = false;
//     let idx = args.len() - 2;
//     let val = &args[idx];
//     // if args.len() < 2 as usize {
//     //     return exist;
//     // }
//     if *val == ">".to_string() || *val == "1>".to_string() {
//         if idx != (args.len() - 2) {
//             print!("Invalid argument after {val}")
//         } else {
//             exist = true;
//         }
//     }
//     return exist;
// }

// fn write_to_std_output(args: &Vec<String>) -> bool {
//     let mut exist = false;
//     let idx = args.len() - 1;
//     let val = &args[idx];
//     // if args.len() < 2 as usize {
//     //     return exist;
//     // }
//     if *val == ">".to_string() || *val == "1>".to_string() {
//         exist = true;
//     }
//     return exist;
// }

pub fn write_to_file(content: String, file: String) {

    let path = Path::new(&file);
    let mut to_write = content.trim_end_matches('\n').to_string();

    // If file exists and is not empty and does not end with '\n', add a newline before writing
    if path.exists() {
        if let Ok(existing) = read_to_string(&file) {
            if !existing.is_empty() {
                to_write = format!("\n{}", to_write);
            }
        }
    }

    match fs::OpenOptions::new()
        .create(true) 
        .append(true) 
        .open(&file)
    {
        Ok(mut f) => {
            if let Err(e) = f.write_all(to_write.as_bytes()) {
                eprintln!("failed to write: {}", e);
            }
        }
        Err(e) => {
            eprintln!("failed to open {}: {}", &file, e);
        }
    }
}
