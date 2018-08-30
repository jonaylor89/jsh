
use std::io;
use std::vec::Vec;
use std::path::Path;
use std::env::set_current_dir;
use std::process::Command;

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

fn launch(argv: Vec<&str>) -> i32 {

    let mut command = Command::new(argv[0]);
    let primed_command = command.args(&argv[1..]);

    if let Ok(mut child) = primed_command.spawn() {
        child.wait().expect("Command failed");
    } else {
        println!("Command not started");
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
    let mut status: i32;

    loop {
    
        print!("[jsh]==>>> ");
        io::stdin().read_line(&mut line).expect("read error");
        let commands = line.clone();
        let argv = commands.rsplit(|c| 
                           c == '\t' ||
                           c == '\r' ||
                           c == '\n').collect();
        status = execute(argv);

        if status == 0 {
            break;    
        }
    }
}
