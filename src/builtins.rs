use std::env;
use std::path;
use std::process;

pub fn echo_cmd(args: Option<&str>) {
    match args {
        Some(text) => println!("{text}"),
        None => (),
    }
}

pub fn pwd_cmd() {
    if let Ok(dir) = env::current_dir() {
        println!("{}", dir.display());
    } else {
        print!("failed to execute pwd command");
    }
}

pub fn cd_cmd(args: Option<&str>) {
    match args {
        Some(dir) => match env::set_current_dir(dir.trim()) {
            Err(result) => println!("cd: {}: No such file or directory", dir.trim()),
            Ok(r) => {}
        },
        None => {}
    }
}

pub fn tilde_cmd() {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("/"));
    let _ = env::set_current_dir(home);
}

pub fn existing_command(commandInput: Vec<&str>) {
    let command = commandInput.get(0).map(|v| *v);
    let l = commandInput.len();
    match command {
        Some(text) => {
            if let Ok(path_var) = env::var("PATH") {
                for dir in path_var.split(':') {
                    let full_path = path::Path::new(dir).join(text);
                    if full_path.exists() && full_path.is_file() {
                        // execute command
                        let mut output = process::Command::new(text)
                            // .output()
                            .args(&commandInput[1..l])
                            .spawn()
                            .expect("command did not executed");
                        let _status = output.wait().unwrap();
                        return;
                    }
                }
            }
            println!("{}: not found", text);
            // }
        }
        _ => {}
    }
}

pub fn type_cmd(args: Option<&str>) {
    match args {
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
