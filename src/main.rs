use anyhow::{Result, bail};
use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

static PROMPT_TEXT: &str = "db";

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    kind: StatementType,
}

fn prompt() -> String {
    let mut line = String::new();
    print!("{} > ", PROMPT_TEXT);
    if let Err(e) = stdout().flush() {
        panic!("Failed to flush stdout: {}", e);
    }
    if let Err(e) = stdin().read_line(&mut line) {
        panic!("Failed to read stdin: {}", e);
    }
    line.trim_end().to_string()
}

fn meta_command_handler(input: &str) -> Result<()> {
    match input {
        ".exit" => exit(0),
        _ => bail!(format!("Unrecognized meta command '{}'", input)),
    }
}

fn prepare_statement(input: &str) -> Result<Statement> {
    let parts = input.split_whitespace();
    let args: Vec<String> = parts.map(String::from).collect();

    match args[0].as_str() {
        "insert" => Ok(Statement {
            kind: StatementType::Insert,
        }),
        "select" => Ok(Statement {
            kind: StatementType::Select,
        }),
        _ => bail!("Unrecognized command '{input}'"),
    }
}

fn execute_statement(stmt: Statement) {
    match stmt.kind {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do an select."),
    }
}

fn main() {
    loop {
        let input = prompt();
        if input.is_empty() {
            continue;
        }

        if input.chars().nth(0) == Some('.') {
            if let Err(e) = meta_command_handler(&input) {
                println!("{e}")
            }
            continue;
        }

        let stmt = match prepare_statement(&input) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        execute_statement(stmt);
    }
}
