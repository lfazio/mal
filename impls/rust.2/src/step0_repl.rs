mod types;

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

use rustyline::DefaultEditor;

#[derive(Debug, Clone)]
enum MalError {
    Break(String),
    Readline(String),
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MalError::Break(s) => write!(f, "Break: {}", s),
            MalError::Readline(s) => write!(f, "Error: {}", s),
        }
    }
}

#[allow(non_snake_case)]
fn READ<'a>(rl: &'a mut DefaultEditor, prompt: &'a str) -> Result<String, MalError> {
    match rl.readline(prompt) {
        Ok(line) => {
            let _ = rl.add_history_entry(line.as_str());

            Ok(line)
        }
        Err(rustyline::error::ReadlineError::Interrupted) => {
            Err(MalError::Break("CTRL-C".to_string()))
        }
        Err(rustyline::error::ReadlineError::Eof) => Err(MalError::Break("CTRL-D".to_string())),
        Err(e) => Err(MalError::Readline(format!("{}", e))),
    }
}

#[allow(non_snake_case)]
fn EVAL(input: Result<String, MalError>) -> Result<String, MalError> {
    input
}

#[allow(non_snake_case)]
fn PRINT(output: Result<String, MalError>) -> bool {
    match output {
        Ok(result) => println!("{}", result),
        Err(MalError::Break(s)) => {
            println!("{}\nBye!", s);

            return false;
        }
        Err(e) => println!("Error: {}", e),
    }

    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    while PRINT(EVAL(READ(&mut rl, "user> "))) {
        continue;
    }

    let _ = rl.save_history(".mal_history.txt");

    Ok(())
}
