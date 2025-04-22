use std::{cell::RefCell, rc::Rc};

// Import (via `use`) the `fmt` module to make it available.
use rustyline::DefaultEditor;

#[macro_use]
mod evaluation;
mod printer;
mod reader;
mod types;

use crate::types::MalVal;
use crate::types::environment::MalEnv;
use crate::types::error::MalError;

#[macro_use]
mod builtins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Rc::new(RefCell::new(MalEnv::new(None)));
    builtins::register(&env);

    let mut args: Vec<MalVal> = vec![];
    if std::env::args().len() > 1 {
        let arg1 = std::env::args().nth(1).clone();

        if std::env::args().len() > 2 {
            for arg in std::env::args().skip(2) {
                args.push(MalVal::Str(arg));
            }
        }
        let _ = env.borrow_mut().set("*ARGV*", &MalVal::List(Rc::new(args)));

        if let Some(file) = arg1 {
            let _ = builtins::re(&format!("(load-file \"{}\")", file), &env);
            std::process::exit(0);
        }
    } else {
        let _ = env.borrow_mut().set("*ARGV*", &MalVal::List(Rc::new(args)));
    }

    let mut rl = DefaultEditor::new()?;
    if rl.load_history(".mal_history.txt").is_err() {
        println!("No previous history.");
    }

    let _ = builtins::re("(def! DEBUG-EVAL 1)", &env);

    // REPL
    while printer::print(evaluation::eval(reader::read(&mut rl, "user> "), &env)) {
        continue;
    }

    let _ = rl.save_history("~/.mal_history.txt");

    Ok(())
}
