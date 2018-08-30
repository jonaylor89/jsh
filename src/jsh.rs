
use std::io;
use std::vec::Vec;
use std::path::Path;
use std::env::set_current_dir;
use std::ffi::CString;
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

fn num_builtins() -> usize {
    return builtin_str.len();
}

/*
 * Shell builtin functions
 */

fn sh_cd(argv: Vec<&str>) -> i32 {
    let path = Path::new(argv[1]);

    if !path.exists() {
        println!("Error");
    } else {
        if set_current_dir(&path).is_err(){
            println!("Cannot change directory");
        }
    }

    return 1;
}

fn sh_help(_argv: Vec<&str>) -> i32 {
    println!("John Naylor");
    println!("Shell heavily influenced by Stephen Brennan's LSH");
    println!("The following are builtin:");

    for i in 0..num_builtins() {
        println!("===> {}", builtin_str[i]);
    }

    println!("Use the `man` command for info on other programs");

    return 1;
}

fn sh_exit(_argv: Vec<&str>) -> i32 {
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

fn vec_to_c_slice(arr: Vec<&str>) -> &[CString] {

    let mut C_arr = Vec::new();

    for v in arr {
        C_arr.push(CString::new(v).unwrap());
    }

    return C_arr.as_slice();

}

fn launch(argv: Vec<&str>) -> i32 {

    let command_C = &CString::new(argv[0]).unwrap();
    let arg_C = vec_to_c_slice(argv);

    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            loop {
                let status = wait::waitpid(child, None);
            }
        }
        ForkResult::Child => {
            if execvp(command_C, arg_C).is_err() {
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

    let mut line = String::new();
    let mut argv: Vec<&str>;
    let mut status: i32;

    loop {
    
        println!("> ");
        io::stdin().read_line(&mut line).unwrap();
        argv = split_line(&line);
        status = execute(argv);

        if status == 0 {
            break;    
        }
    }
}
