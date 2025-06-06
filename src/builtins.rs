
pub fn echo_cmd(args: Option<&str>) {
    match args {
        Some(text) => println!("{text}"),
        None => (),
    }
}

pub fn type_cmd(args: Option<&str>) {
    match args {
        Some(text) => {
            let v: Vec<&str> = text.split_ascii_whitespace().collect();
            // if v.l
            if v.len() > 1 {
                println!("Too  many arguments");
            }
            else {
                match v[0] {
                    "exit" | "echo" | "type" => println!("{} is builtin", v[0]),
                    _ => println!("{}: not found", v[0]),
                }
            }
        }
        None => (),
    }
}
