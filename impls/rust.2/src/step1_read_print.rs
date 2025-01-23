// Import (via `use`) the `fmt` module to make it available.
use rustyline::DefaultEditor;

mod printer;
mod reader;

mod error;
use error::MalError;

mod types;
use types::MalVal;

#[allow(non_snake_case)]
fn EVAL(input: Result<MalVal, MalError>) -> Result<MalVal, MalError> {
    input
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    // REPL
    while printer::print(EVAL(reader::read(&mut rl, "user> "))) {
        continue;
    }

    let _ = rl.save_history(".mal_history.txt");

    Ok(())
}
