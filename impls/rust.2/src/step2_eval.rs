use std::{cell::RefCell, rc::Rc};

// Import (via `use`) the `fmt` module to make it available.
use rustyline::DefaultEditor;

mod evaluation;
mod printer;
mod reader;
use types::environment::MalEnv;

mod types;
use types::MalVal;
use types::error::MalError;

#[macro_use]
mod builtins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    let env = Rc::new(RefCell::new(MalEnv::new(None)));
    builtins::register(&env);

    // REPL
    while printer::print(evaluation::eval(reader::read(&mut rl, "user> "), &env)) {
        continue;
    }

    let _ = rl.save_history(".mal_history.txt");

    Ok(())
}
