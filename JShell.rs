
use std::io;

fn sh_help(argv: Vec[str]) ->i32 {
    println!("John Naylor");
    println!("Shell heavily influenced by Stephen Brennan's LSH");
    println!("The following are builtin:");

    for i in 0..num_builtins() {
        println!("===> ", builtin_str[i]);
    }

    println!("Use the `man` command for info on other programs");

    return 1;
}

fn sh_exit(argv: Vec[str]) -> i32 {
    return 0;
}

/*
 * jsh Parsing and Execution
 */

fn split_line(line: str) -> Vec[str] {

}

fn launch(argv: Vec[str]) -> i32 {

}

fn execute(argv: Vec[str]) -> i32 {

    if argv[0] == None {
        return 1; 
    }

    for i in 0..num_builtins() {
        if argv[0] == builtin_str[i] {
            return builtin_func[i](argv);
        }
    }

    return launch(argv);
}

fn shell_loop() {

    let mut line = String::new();
    let mut argv: Vec[str];
    let mut status: i32;

    loop {
    
        println!("> ");
        line = io::stdin.read_line().unwraps();
        argv = split_line(line);
        status = execute(args);

        if !status {
            break;    
        }
    }
}

fn main() {

    shell_shell();

}
