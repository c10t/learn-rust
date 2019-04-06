use std::io;
use std::io::Write;
use std::process;

fn main() {
    // Not implemented input buffer: https://cstack.github.io/db_tutorial/parts/part1.html
    let mut input_buffer = String::new();
    loop {
        print_prompt();
        // https://stackoverflow.com/questions/34993744/why-does-this-read-input-before-printing
        io::stdout().flush().expect("flush failed!");
        io::stdin().read_line(&mut input_buffer).expect("failed to read line");

        // command.trim_end() : &str
        input_buffer = input_buffer.trim_end().to_string();

        if input_buffer.starts_with(".") {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::Exit => {
                    // https://doc.rust-lang.org/std/process/fn.exit.html
                    process::exit(0);
                }
                MetaCommandResult::Success => {
                    input_buffer.clear();
                    continue;
                },
                MetaCommandResult::UnrecognizedCommand(msg) => {
                    println!("{}", msg);
                    input_buffer.clear();
                    continue;
                },          
            };
        }

        match prepare_statement(&input_buffer) {
            PrepareResult::Success(statement) => {
                execute_statement(&statement);
            },
            PrepareResult::UnrecognizedStatement(msg) => {
                println!("{}", msg);
                input_buffer.clear();
                continue;
            },
        };
        input_buffer.clear();

        println!("Executed");
    }
}

enum MetaCommandResult {
    Success,
    Exit,
    UnrecognizedCommand(String)
}

enum PrepareResult {
    Success(Statement),
    UnrecognizedStatement(String),
}

fn print_prompt() {
    print!("db > ");
}

fn do_meta_command(input_buffer: &str) -> MetaCommandResult {
    if input_buffer == ".exit" {
        MetaCommandResult::Exit
    } else if input_buffer == ".test" {
        MetaCommandResult::Success
    } else {
        MetaCommandResult::UnrecognizedCommand(format!("Unrecognized keyword at start of '{}'", input_buffer))
    }
}

enum StatementMethod {
    Insert,
    Select,
}

struct Statement {
    method: StatementMethod
}

fn prepare_statement(input_buffer: &str) -> PrepareResult {
    if input_buffer.starts_with("insert") {
        let statement = Statement { method: StatementMethod::Insert };
        PrepareResult::Success(statement)
    } else if input_buffer.starts_with("select") {
        let statement = Statement { method: StatementMethod::Select };
        PrepareResult::Success(statement)
    } else {
        PrepareResult::UnrecognizedStatement(format!("Unrecognized command '{}'", input_buffer))
    }
}

fn execute_statement(statement: &Statement) {
    match statement.method {
        StatementMethod::Insert => {
            println!("This is where we would do an insert.");
        },
        StatementMethod::Select => {
            println!("This is where we would do a select.");
        },
    }
}
