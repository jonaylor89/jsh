use std::io::{self, Write};
use std::vec::Vec; use std::path::Path;
use std::env::set_current_dir;
use std::process::Command;
use prompt;

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

    if argv.len() < 2 {
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

    if argv.len() < 1 {
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

        print!("{} ", prompt::prompt());
        io::stdout().flush().ok().expect("error flushing buffer");

        line.clear();
        io::stdin().read_line(&mut line).expect("read error");

        // Make sure line isn't empty
        if line.trim().is_empty() {
            continue
        }

        let commands = line.clone();
        let argv = commands.trim().split_whitespace().collect();
        status = execute(argv);

        if status == 0 {
            break
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_execute_none() {
        use jsh::execute;

        let args = vec![];

        let result = execute(args);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_execute_exit() {
        use jsh::execute;

        let args = vec!["exit"];

        let result = execute(args);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_execute_cd() {
        use jsh::execute;

        let args = vec!["cd", "test"];

        let result = execute(args);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_execute_cd_no_dir() {
        use jsh::execute;

        let args = vec!["cd"];

        let result = execute(args);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_execute_help() {
        use jsh::execute;

        let args = vec!["help"];

        let result = execute(args);

        assert_eq!(result, 1);
    }
}
