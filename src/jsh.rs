
use std::io;
use std::vec::Vec;
use std::path::Path;
use std::env::set_current_dir;
use nix::sys::signal::*;
use nix::sys::wait;
use nix::unistd::*;

static builtin_str: &'static [&'static str; 3] = &[
    "cd",
    "help",
    "exit",
];

static builtin_func: &'static [fn(Vec<&str>) -> i32; 3] = &[
    sh_cd,
    sh_help,
    sh_exit,
];

fn num_builtins() -> i32 {
    return builtin_str.len();
}

/*
 * Shell builtin functions
 */

fn sh_cd(argv: Vec<&str>) -> i32 {
    let path = Path::new(argv[1]);

    if !path.exists {
        println!("Error");
    } else {
        if set_current_dir(&path).is_err(){
            println!("Cannot change directory");
        }
    }

    return 1;
}

fn sh_help(argv: Vec<&str>) -> i32 {
    println!("John Naylor");
    println!("Shell heavily influenced by Stephen Brennan's LSH");
    println!("The following are builtin:");

    for i in 0..num_builtins() {
        println!("===> {}", builtin_str[i]);
    }

    println!("Use the `man` command for info on other programs");

    return 1;
}

fn sh_exit(argv: Vec<&str>) -> i32 {
    return 0;
}

/*
 * jsh Parsing and Execution
 */

fn split_line(line: &str) -> Vec<&str> {
    return line.rsplit(|c| 
                       c == '\t' ||
                       c == '\r' ||
                       c == '\n').collect();
}

fn launch(argv: Vec<&str>) -> i32 {
    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            let wpid = wait::waitpid(child);
        }
        ForkResult::Child => {
            if execvp(argv[0], argv).is_err() {
                println!("Execution error") ;
            }

            return 1;
        }
    }

    return 1;
}

fn execute(argv: Vec<&str>) -> i32 {

    if argv[0].is_empty() {
        return 1; 
    }

    for i in 0..num_builtins() {
        if argv[0] == builtin_str[i] {
            return builtin_func[i](argv);
        }
    }

    return launch(argv);
}

pub fn shell_loop() {

    let mut line = &String::new();
    let mut argv: Vec<&str>;
    let mut status: i32;

    loop {
    
        println!("> ");
        line = io::Stdin().read_line().unwraps();
        argv = split_line(line);
        status = execute(argv);

        if status == 0 {
            break;    
        }
    }
}
