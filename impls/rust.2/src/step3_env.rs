use std::{cell::RefCell, rc::Rc};

// Import (via `use`) the `fmt` module to make it available.
use rustyline::DefaultEditor;

mod environment;
mod evaluation;
mod printer;
mod reader;
use environment::MalEnv;

mod error;
use error::MalError;

mod types;
use types::MalVal;

mod builtins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    let env = Rc::new(RefCell::new(MalEnv::new(None)));
    builtins::register(env.clone());

    // REPL
    while printer::print(evaluation::eval(
        reader::read(&mut rl, "user> "),
        env.clone(),
    )) {
        continue;
    }

    let _ = rl.save_history(".mal_history.txt");

    Ok(())
}
