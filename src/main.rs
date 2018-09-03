
extern crate termion;

mod jsh;
mod prompt;

use termion::{color, style}

fn main() {

    println!();
    println!("{}Welcome to jsh!!!{}", style::Bold, style::Reset);
    println!("Use the `{}help{}` for information about the shell", color::Fg(color::blue), color::Fg(color::Reset));
    println!();

    jsh::shell_loop();

}
