mod shell;
mod builtins;
mod util;
fn main() {
    // for argument in env::args() {
    //     println!("{argument}");
    // }
    // match env::var("PATH") {
    //     Ok(v)=> println!("{v}"),
    //     Err(e)=>println!("error"),
    // }
    shell::run();
    
}
