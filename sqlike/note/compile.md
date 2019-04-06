error: expected identifier, found keyword `type`
  --> src\main.rs:80:5
   |
80 |     type: StatementType
   |     ^^^^ expected identifier, found keyword
help: you can escape reserved keywords to use them as identifiers
   |
80 |     r#type: StatementType
   |     ^^^^^^

error: expected identifier, found keyword `type`
  --> src\main.rs:85:33
   |
85 |         statement = Statement { type: STATEMENT_INSERT };
   |                     ---------   ^^^^ expected identifier, found keyword
   |                     |
   |                     while parsing this struct
help: you can escape reserved keywords to use them as identifiers
   |
85 |         statement = Statement { r#type: STATEMENT_INSERT };
   |                                 ^^^^^^

error: expected identifier, found keyword `type`
  --> src\main.rs:88:33
   |
88 |         statement = Statement { type: STATEMENT_SELECT };
   |                     ---------   ^^^^ expected identifier, found keyword
   |                     |
   |                     while parsing this struct
help: you can escape reserved keywords to use them as identifiers
   |
88 |         statement = Statement { r#type: STATEMENT_SELECT };
   |                                 ^^^^^^

error: expected identifier, found keyword `type`
  --> src\main.rs:96:21
   |
96 |     match statement.type {
   |                     ^^^^ expected identifier, found keyword
help: you can escape reserved keywords to use them as identifiers
   |
96 |     match statement.r#type {
   |                     ^^^^^^

error: format argument must be a string literal
  --> src\main.rs:35:26
   |
35 |                 println!(msg);
   |                          ^^^
help: you might be missing a string literal to format with
   |
35 |                 println!("{}", msg);
   |                          ^^^^^

error[E0532]: expected unit struct/variant or constant, found tuple variant `META_COMMAND_RESULT::UNRECOGNIZED_COMMAND`
  --> src\main.rs:23:17
   |
23 |                 META_COMMAND_RESULT::UNRECOGNIZED_COMMAND => {
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a unit struct/variant or constant

error[E0532]: expected tuple struct/variant, found unit variant `PREPARE_RESULT::UNRECOGNIZED_STATEMENT`
  --> src\main.rs:34:13
   |
34 |             PREPARE_RESULT::UNRECOGNIZED_STATEMENT(msg) => {
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a tuple struct/variant

error[E0425]: cannot find value `command` in this scope
  --> src\main.rs:65:8
   |
65 |     if command == ".exit" {
   |        ^^^^^^^ not found in this scope

error[E0425]: cannot find value `SUCCESS` in this scope
  --> src\main.rs:68:9
   |
68 |         SUCCESS
   |         ^^^^^^^ not found in this scope
help: possible candidates are found in other modules, you can import them into scope
   |
1  | use crate::META_COMMAND_RESULT::SUCCESS;
   |
1  | use crate::PREPARE_RESULT::SUCCESS;
   |

error[E0425]: cannot find function `UNRECOGNIZED_COMMAND` in this scope
  --> src\main.rs:70:9
   |
70 |         UNRECOGNIZED_COMMAND(format!("Unrecognized keyword at start of '{}'", input_buffer))
   |         ^^^^^^^^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
1  | use crate::META_COMMAND_RESULT::UNRECOGNIZED_COMMAND;
   |

error[E0425]: cannot find value `SUCCESS` in this scope
  --> src\main.rs:86:9
   |
86 |         SUCCESS
   |         ^^^^^^^ not found in this scope
help: possible candidates are found in other modules, you can import them into scope
   |
1  | use crate::META_COMMAND_RESULT::SUCCESS;
   |
1  | use crate::PREPARE_RESULT::SUCCESS;
   |

error[E0425]: cannot find value `SUCCESS` in this scope
  --> src\main.rs:89:9
   |
89 |         SUCCESS
   |         ^^^^^^^ not found in this scope
help: possible candidates are found in other modules, you can import them into scope
   |
1  | use crate::META_COMMAND_RESULT::SUCCESS;
   |
1  | use crate::PREPARE_RESULT::SUCCESS;
   |

error[E0425]: cannot find value `UNRECOGNIZED_STATEMENT` in this scope
  --> src\main.rs:91:9
   |
91 |         UNRECOGNIZED_STATEMENT
   |         ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
   |
1  | use crate::PREPARE_RESULT::UNRECOGNIZED_STATEMENT;
   |

warning: type `META_COMMAND_RESULT` should have a camel case name
  --> src\main.rs:50:6
   |
50 | enum META_COMMAND_RESULT {
   |      ^^^^^^^^^^^^^^^^^^^ help: convert the identifier to camel case: `MetaCommandResult`
   |
   = note: #[warn(non_camel_case_types)] on by default

warning: variant `UNRECOGNIZED_COMMAND` should have a camel case name
  --> src\main.rs:52:5
   |
52 |     UNRECOGNIZED_COMMAND(String)
   |     ^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to camel case: `UnrecognizedCommand`

warning: type `PREPARE_RESULT` should have a camel case name
  --> src\main.rs:55:6
   |
55 | enum PREPARE_RESULT {
   |      ^^^^^^^^^^^^^^ help: convert the identifier to camel case: `PrepareResult`

warning: variant `UNRECOGNIZED_STATEMENT` should have a camel case name
  --> src\main.rs:57:5
   |
57 |     UNRECOGNIZED_STATEMENT,
   |     ^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to camel case: `UnrecognizedStatement`

warning: variant `STATEMENT_INSERT` should have a camel case name
  --> src\main.rs:75:5
   |
75 |     STATEMENT_INSERT,
   |     ^^^^^^^^^^^^^^^^ help: convert the identifier to camel case: `StatementInsert`

warning: variant `STATEMENT_SELECT` should have a camel case name
  --> src\main.rs:76:5
   |
76 |     STATEMENT_SELECT,
   |     ^^^^^^^^^^^^^^^^ help: convert the identifier to camel case: `StatementSelect`

error[E0308]: mismatched types
  --> src\main.rs:18:35
   |
18 |             match do_meta_command(input_buffer) {
   |                                   ^^^^^^^^^^^^
   |                                   |
   |                                   expected &str, found struct `std::string::String`
   |                                   help: consider borrowing here: `&input_buffer`
   |
   = note: expected type `&str`
              found type `std::string::String`

error[E0308]: mismatched types
  --> src\main.rs:32:33
   |
32 |         match prepare_statement(input_buffer, &mut statement) {
   |                                 ^^^^^^^^^^^^
   |                                 |
   |                                 expected &str, found struct `std::string::String`
   |                                 help: consider borrowing here: `&input_buffer`
   |
   = note: expected type `&str`
              found type `std::string::String`

error[E0308]: mismatched types
  --> src\main.rs:32:47
   |
32 |         match prepare_statement(input_buffer, &mut statement) {
   |                                               ^^^^^^^^^^^^^^ expected struct `Statement`, found struct `std::string::String`
   |
   = note: expected type `&mut Statement`
              found type `&mut std::string::String`

error[E0308]: mismatched types
  --> src\main.rs:43:27
   |
43 |         execute_statement(&statement);
   |                           ^^^^^^^^^^ expected struct `Statement`, found struct `std::string::String`
   |
   = note: expected type `&Statement`
              found type `&std::string::String`

warning: unreachable expression
  --> src\main.rs:68:9
   |
68 |         SUCCESS
   |         ^^^^^^^
   |
   = note: #[warn(unreachable_code)] on by default

error[E0063]: missing field `type` in initializer of `Statement`
  --> src\main.rs:85:21
   |
85 |         statement = Statement { type: STATEMENT_INSERT };
   |                     ^^^^^^^^^ missing `type`

error[E0308]: mismatched types
  --> src\main.rs:85:21
   |
85 |         statement = Statement { type: STATEMENT_INSERT };
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                     |
   |                     expected mutable reference, found struct `Statement`
   |                     help: consider mutably borrowing here: `&mut Statement { type: STATEMENT_INSERT }`
   |
   = note: expected type `&mut Statement`
              found type `Statement`

error[E0063]: missing field `type` in initializer of `Statement`
  --> src\main.rs:88:21
   |
88 |         statement = Statement { type: STATEMENT_SELECT };
   |                     ^^^^^^^^^ missing `type`

error[E0308]: mismatched types
  --> src\main.rs:88:21
   |
88 |         statement = Statement { type: STATEMENT_SELECT };
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                     |
   |                     expected mutable reference, found struct `Statement`
   |                     help: consider mutably borrowing here: `&mut Statement { type: STATEMENT_SELECT }`
   |
   = note: expected type `&mut Statement`
              found type `Statement`

error: aborting due to 21 previous errors

Some errors occurred: E0063, E0308, E0425, E0532.
For more information about an error, try `rustc --explain E0063`.
error: Could not compile `sqlike`.

To learn more, run the command again with --verbose.