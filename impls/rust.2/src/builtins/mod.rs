use std::fs;
use std::io::Read;
use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

use crate::evaluation;
use crate::printer;
use crate::reader;

mod core;
mod math;
mod string;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    core::register(env);
    string::register(env);
    math::register(env);

    env.borrow_mut().set(
        "slurp",
        &MalVal::Function(|args| {
            if args.len() > 1 {
                return Err(MalError::Error(
                    "read-string expects only one string".to_string(),
                ));
            }

            if let Some(MalVal::Str(path)) = args.first() {
                let mut file = if let Ok(fd) = fs::File::open(path) {
                    fd
                } else {
                    return Err(MalError::Error(format!("Failed to open file {}", path)));
                };
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(MalError::Error(format!(
                            "Failed to read file {}: {}",
                            path, e
                        )));
                    }
                }
                Ok(MalVal::Str(contents))
            } else {
                Err(MalError::Error(
                    "slurp expects one argument of string type".to_string(),
                ))
            }
        }),
    );

    let _ = re("(def! not (fn* (a) (if a false true)))", env);
    let _ = re(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\nnil)\")))))",
        env,
    );
}

pub fn re(line: &str, env: &Rc<RefCell<MalEnv>>) -> MalReturn {
    evaluation::eval(reader::read_str(line), env)
}

pub fn rep(line: &str, env: &Rc<RefCell<MalEnv>>) -> bool {
    printer::print(evaluation::eval(reader::read_str(line), env))
}
