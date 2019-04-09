use std::io;
use std::io::Write;
use std::process;

#[macro_use] extern crate scan_fmt;

// REMAIN TODO:
// - Re-consider ownership
// - Re-consider input buffer
// - Extract to lib.rs
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
            PrepareResult::SyntaxError(msg) => {
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
    SyntaxError(String)
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
    Insert(Row),
    Select,
}

struct Statement {
    method: StatementMethod
}

// const uint32_t COLUMN_USERNAME_SIZE = 32;
// const uint32_t COLUMN_EMAIL_SIZE = 255;
// struct Row_t {
//   uint32_t id;
//   char username[COLUMN_USERNAME_SIZE];
//   char email[COLUMN_EMAIL_SIZE];
// };
// typedef struct Row_t Row;
struct Row {
    id: usize,
    username: String, // varchar(32)
    email: String // varchar(255)
}

fn prepare_statement(input_buffer: &str) -> PrepareResult {
    if input_buffer.starts_with("insert") {
        // use scan_fmt macro https://docs.rs/scan_fmt/0.1.3/scan_fmt/
        let (id, username, email) = scan_fmt!(input_buffer, "insert {d} {} {}", usize, String, String);
        match (id, username, email) {
            (Some(id), Some(username), Some(email)) => {
                let row_to_insert = Row { id, username, email };
                let statement = Statement { method: StatementMethod::Insert(row_to_insert) };
                PrepareResult::Success(statement)
            },
            _ => PrepareResult::SyntaxError(String::from("Invalid Arguments"))
        }
    } else if input_buffer.starts_with("select") {
        let statement = Statement { method: StatementMethod::Select };
        PrepareResult::Success(statement)
    } else {
        PrepareResult::UnrecognizedStatement(format!("Unrecognized command '{}'", input_buffer))
    }
}

fn execute_statement(statement: &Statement) {
    match &statement.method {
        StatementMethod::Insert(_row) => {
            println!("This is where we would do an insert.");
        },
        StatementMethod::Select => {
            println!("This is where we would do a select.");
        },
    }
}
