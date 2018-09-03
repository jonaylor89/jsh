
extern crate termion;

mod jsh;
mod prompt;

fn main() {

    println!();
    println!("Welcome to jsh!!!");
    println!();

    jsh::shell_loop();

}
