
use std::io::{self, Write};
use std::vec::Vec; use std::path::Path;
use std::env::set_current_dir;
use std::process::Command;

/*
 * Static constants
 */

static BUILTIN_STR: &'static [&'static str; 3] = &[
    "cd",
    "help",
    "exit",
];

static BUILTIN_FUNC: &'static [fn(Vec<&str>) -> i32; 3] = &[
    jsh_cd,
    jsh_help,
    jsh_exit,
];

fn num_builtins() -> usize {
    return BUILTIN_STR.len();
}

/*
 * Shell builtin functions
 */

fn jsh_cd(argv: Vec<&str>) -> i32 {

    if !argv[1].is_empty() {
        println!("cd needs a directory");
        return 1; 
    }

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

fn jsh_help(_argv: Vec<&str>) -> i32 {
    println!();
    println!("John Naylor");
    println!("Shell heavily influenced by Stephen Brennan's LSH");
    println!("The following are builtin:");

    for i in 0..num_builtins() {
        println!("===> {}", BUILTIN_STR[i]);
    }

    println!("Use the `man` command for info on other programs");
    println!();

    return 1;
}

fn jsh_exit(_argv: Vec<&str>) -> i32 {
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
        println!("Command doesn't exist");
    }

    return 1;
}

fn execute(argv: Vec<&str>) -> i32 {

    if argv[0].is_empty() {
        return 1; 
    }

    for i in 0..num_builtins() {
        if argv[0] == BUILTIN_STR[i] {
            return BUILTIN_FUNC[i](argv);
        }
    }

    return launch(argv);
}

pub fn shell_loop() {

    let mut line = String::new();
    let mut status: i32;

    loop {
    
        print!("[jsh]==>>> ");
        io::stdout().flush().ok().expect("error flushing buffer");

        line.clear();
        io::stdin().read_line(&mut line).expect("read error");

        if line.trim().is_empty() {
            continue; 
        }

        let commands = line.clone();
        let argv = commands.trim().split_whitespace().collect();
        status = execute(argv);

        if status == 0 {
            break;    
        }
    }
}
