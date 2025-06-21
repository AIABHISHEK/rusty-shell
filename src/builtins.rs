use crate::shell::RedirectType;
use std::env;
use std::env::args;
use std::fs;
use std::fs::read_to_string;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path;
use std::path::Path;
use std::process;
use std::process::Child;
use std::process::Output;
use std::process::Stdio;
use std::thread::spawn;

pub fn echo_cmd(args: &Vec<String>, output_: &mut Vec<String>) {
    // if !*redirect {
    //     println!("{}", args.join(" "));
    // }
    output_.push(args.join(" "));
}

pub fn pwd_cmd(output_: &mut Vec<String>) {
    if let Ok(dir) = env::current_dir() {
        println!("{}", dir.display());
    } else {
        print!("failed to execute pwd command");
    }
}

pub fn cd_cmd(args: &Vec<String>, output_: &mut Vec<String>) {
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

pub fn existing_command(
    command: &str,
    args: &Vec<String>,
    output_: &mut Vec<String>,
    err_: &mut String,
    redirect: &mut RedirectType,
) {
    // check if have > or 1> in last second index then output should be
    // let exist = write_to_file_arg_exist(args);
    // println!("this is exisitng command");
    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            let full_path = dir.join(command);
            // print!("path : {}", full_path.display());
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

                // if output.status.success() {
                let out = &output.stdout;

                let so = String::from_utf8_lossy(out.as_slice());
                let so = so.to_string();
                // println!("so {}", so);
                // if output_.len() > 0 {
                // output_.push(format!("\n{}", so));
                // }
                // else {
                output_.push(so);
                // }
                // *redirect = true;
                // }
                // else {
                let se = String::from_utf8_lossy(&output.stderr.as_slice());
                let se = se.to_string();

                // println!("{}", s);
                *err_ = se;
                // print!("{}")
                // }
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

fn type_exist(text: &String) -> String {
    // print!("this is {text}");
    let v: Vec<&str> = text.split_ascii_whitespace().collect();
    // if v.len() > 1 {
    //     format!("Too  many arguments");
    // } else {
    match text.as_str() {
        "exit" | "echo" | "type" | "pwd" | "cd" => {
            let a = format!("{} is a shell builtin", text);
            // return;
            return a;
        }
        _ => {}
    }
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let full_path = path::Path::new(dir).join(text);
            // println!("this is full path: {:?}", full_path);
            // println!("this is: {dir}");
            if full_path.exists() && full_path.is_file() {
                // execute command

                let a = format!("{} is {}", text, full_path.display());

                return a;
            }
        }
    }
    let b = format!("{}: not found", text);
    return b;
    // }
}

pub fn handle_pipe(
    input: &Vec<String>,
    output_: &mut Vec<String>,
    err_: &mut String,
    redirect: &mut RedirectType,
) {
    let mut commands = Vec::new();
    let mut current = Vec::new();
    for token in input {
        if token == "|" {
            if !current.is_empty() {
                commands.push(current);
                current = Vec::new();
            }
        } else {
            current.push(token.clone());
        }
    }
    if !current.is_empty() {
        commands.push(current);
    }
    let mut processes: Vec<Child> = Vec::new();
    let mut prev_stdout: Option<process::ChildStdout> = None;
    let mut prev_builtin_output: Option<String> = None;

    for (i, cmd) in commands.iter().enumerate() {
        //
        // println!("inside ");
        match cmd[0].as_str() {
            "type" => {
                let mut type_args: Vec<String> = Vec::new();

                if cmd.len() > 1 {
                    type_args.push(cmd[1].clone());
                }
                // command.args(&cmd[1..]);
                let mut buf = String::new();
                if let Some(mut stdin) = prev_stdout.take() {
                    // command.stdin(stdin);
                    stdin
                        .read_to_string(&mut buf)
                        .expect("failed to read from stdin");
                }
                if !buf.trim().is_empty() {
                    type_args.push(buf.trim().to_string());
                } else if let Some(out) = prev_builtin_output {
                    type_args.push(out.clone());
                    // prev_builtin_output = None;
                }
                // }
                // if type_args.len() >=1 {
                let b = type_exist(&type_args[0]);
                // println!("this is type output {}", b);
                prev_builtin_output = Some(b);
                // }
                // println!("{} out length of  {}", i, commands.len());
                if i == commands.len() - 1 {
                    // println!("using output {}", so);
                    if let Some(ref so) = prev_builtin_output {
                        // println!("{} length of  {} is , {}", i, commands.len(), so);
                        output_.clear();
                        output_.push(so.clone());
                    }
                }
            }
            "echo" => {
                let mut echo_args: Vec<String> = Vec::new();
                let mut buf = String::new();
                if cmd.len() > 1 {
                    // command.args(&cmd[1..]);
                    echo_args.append(&mut cmd[1..].to_vec());
                }
                if let Some(mut stdin) = prev_stdout.take() {
                    // command.stdin(stdin);
                    stdin
                        .read_to_string(&mut buf)
                        .expect("failed to read from stdin");
                    if !buf.trim().is_empty() {
                        echo_args.push(buf);
                    }
                }
                if let Some(out) = prev_builtin_output {
                    echo_args.push(out.clone());
                    // prev_builtin_output = None;
                }

                echo_cmd(&echo_args, output_);
                // print!("this is {}", output_.join(" "));
                // prev_builtin_output = Some(output_.join(" "));
                prev_builtin_output = Some(format!("{}\n",output_.join(" ")));
                if i != commands.len() - 1 {
                    // println!("{} output {}", i, commands.len());
                    // if let Some(ref so) = prev_builtin_output {
                        output_.clear();
                        // output_.push(so.clone());
                    // }
                }
            }
            (som) => {
                let mut command = process::Command::new(&cmd[0]);
                if cmd.len() > 1 {
                    command.args(&cmd[1..]);
                }

                // Set stdin for this command
                // prev cmd out will be std out fro current
                if let Some(ref output) = prev_builtin_output {
                    command.stdin(Stdio::piped());
                    // prev_builtin_output = None;
                } else if let Some(stdin) = prev_stdout.take() {
                    command.stdin(stdin);
                }

                // For all but the last command, pipe stdout
                if i < commands.len() - 1 {
                    command.stdout(Stdio::piped());
                }

                let mut child = command.spawn().expect("failed to spawn process");
                if let Some(output) = prev_builtin_output.take() {
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin.write_all(output.as_bytes()).unwrap();
                    }
                }
                prev_builtin_output = None;
                // Save the stdout to pass as stdin to the next command
                if i < commands.len() - 1 {
                    prev_stdout = Some(child.stdout.take().expect("failed to get stdout"));
                    processes.push(child);
                } else {
                    // last command
                    // if let Some(ref so) = prev_builtin_output {
                    //     println!("using output {}", so);
                    //     output_.push(so.clone());
                    // } else {
                    let output = &child.wait_with_output().expect("failed to wait on child");
                    let so = String::from_utf8_lossy(&output.stdout);
                    let so = so.to_string();
                    output_.push(so);
                    // }
                }
            }
        }
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

pub fn write_to_file(content: String, file: String, append: bool) {
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
        .write(true)
        .truncate(!append)
        .append(append)
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
