use std::io::{Write, stdin, stdout};

static PROMPT_TEXT: &str = "db";

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

fn main() {
    loop {
        let input = prompt();
        if input.as_str() == ".exit" {
            break;
        } else {
            println!("Invalid command");
        }
    }
}
