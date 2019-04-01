use std::io;
use std::io::Write;
use std::process;

fn main() {
    // Not implemented input buffer: https://cstack.github.io/db_tutorial/parts/part1.html
    let mut command = String::new();
    loop {
        print_prompt();
        // https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
        io::stdout().flush().expect("flush failed!");
        io::stdin().read_line(&mut command).expect("failed to read line");

        // command.trim_end() : &str
        command = command.trim_end().to_string();

        if command == ".exit" {
            // https://doc.rust-lang.org/std/process/fn.exit.html
            process::exit(0);
        } else {
            println!("Unrecognized command '{}'", command);
        }
        command.clear();
    }
}

fn print_prompt() {
    print!("db > ");
}
