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
            match do_meta_command(input_buffer) {
                META_COMMAND_RESULT::SUCCESS => {
                    input_buffer.clear();
                    continue;
                },
                META_COMMAND_RESULT::UNRECOGNIZED_COMMAND => {
                    println!("Unrecognized command '{}'", input_buffer);
                    input_buffer.clear();
                    continue;
                },          
            };
        }

        let mut statement = String::new();
        match prepare_statement(input_buffer, &mut statement) {
            PREPARE_RESULT::SUCCESS => (),
            PREPARE_RESULT::UNRECOGNIZED_STATEMENT(msg) => {
                println!(msg);
                input_buffer.clear();
                statement.clear();
                continue;
            }
        };
        input_buffer.clear();

        execute_statement(&statement);

        println!("Executed");
        statement.clear();
    }
}

enum META_COMMAND_RESULT {
    SUCCESS,
    UNRECOGNIZED_COMMAND(String)
}

enum PREPARE_RESULT {
    SUCCESS,
    UNRECOGNIZED_STATEMENT,
}

fn print_prompt() {
    print!("db > ");
}

fn do_meta_command(input_buffer: &str) -> META_COMMAND_RESULT {
    if command == ".exit" {
        // https://doc.rust-lang.org/std/process/fn.exit.html
        process::exit(0);
        SUCCESS
    } else {
        UNRECOGNIZED_COMMAND(format!("Unrecognized keyword at start of '{}'", input_buffer))
    }
}

enum StatementType {
    STATEMENT_INSERT,
    STATEMENT_SELECT,
}

struct Statement {
    type: StatementType
}

fn prepare_statement(input_buffer: &str, statement: &mut Statement) -> PREPARE_RESULT {
    if input_buffer.starts_with("insert") {
        statement = Statement { type: STATEMENT_INSERT };
        SUCCESS
    } else if input_buffer.starts_with("select") {
        statement = Statement { type: STATEMENT_SELECT };
        SUCCESS
    } else {
        UNRECOGNIZED_STATEMENT
    }
}

fn execute_statement(statement: &Statement) {
    match statement.type {
        STATEMENT_INSERT => {
            println!("This is where we would do an insert.");
        },
        STATEMENT_SELECT => {
            println!("This is where we would do a select.");
        },
    }
}
