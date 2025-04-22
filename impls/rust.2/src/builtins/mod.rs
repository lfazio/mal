use std::fs;
use std::io::Read;
use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

use crate::evaluation;
use crate::printer;
use crate::reader;

#[macro_export]
macro_rules! builtin_register {
    ($env:expr, $name:expr, $func:expr) => {
        let _ = $env.borrow_mut().set($name, &MalVal::Function($func));
    };
}

mod core;
mod exceptions;
mod math;
mod string;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    core::register(env);
    exceptions::register(env);
    string::register(env);
    math::register(env);

    builtin_register!(env, "slurp", slurp);

    let _ = re("(def! not (fn* (a) (if a false true)))", env);
    let _ = re(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\nnil)\")))))",
        env,
    );
    let _ = re(
        "(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw \"odd number of forms to cond\")) (cons 'cond (rest (rest xs)))))))",
        env,
    );
}

pub fn re(line: &str, env: &Rc<RefCell<MalEnv>>) -> MalReturn {
    evaluation::eval(reader::read_str(line), env)
}

pub fn rep(line: &str, env: &Rc<RefCell<MalEnv>>) -> bool {
    printer::print(evaluation::eval(reader::read_str(line), env))
}

fn slurp(args: &[MalVal]) -> MalReturn {
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
}

fn quasiquote_iter(list: &Rc<Vec<MalVal>>) -> MalReturn {
    let mut new_list = vec![];

    for elt in list.iter().rev() {
        if let MalVal::List(seq) = elt {
            if seq.len() == 2 && seq[0] == MalVal::Symbol("splice-unquote".to_string()) {
                new_list = vec![
                    MalVal::Symbol("concat".to_string()),
                    seq[1].clone(),
                    MalVal::List(Rc::new(new_list)),
                ];
                continue;
            }
        }

        new_list = vec![
            MalVal::Symbol("cons".to_string()),
            quasiquote(elt)?,
            MalVal::List(Rc::new(new_list.clone())),
        ];
    }

    Ok(MalVal::List(Rc::new(new_list)))
}

pub fn quasiquote(ast: &MalVal) -> MalReturn {
    let mut new_list = vec![];

    if let MalVal::List(l) = ast {
        if l.len() == 2 && l[0] == MalVal::Symbol("unquote".to_string()) {
            return Ok(l[1].clone());
        }

        quasiquote_iter(l)
    } else if let MalVal::Symbol(_) = ast {
        new_list.insert(0, ast.clone());
        new_list.insert(0, MalVal::Symbol("quote".to_string()));

        Ok(MalVal::List(Rc::new(new_list)))
    } else if let MalVal::Hashmap(_) = ast {
        new_list = vec![MalVal::Symbol("quote".to_string()), ast.clone()];

        Ok(MalVal::List(Rc::new(new_list)))
    } else if let MalVal::Vector(seq) = ast {
        new_list = vec![MalVal::Symbol("vec".to_string()), quasiquote_iter(seq)?];

        Ok(MalVal::List(Rc::new(new_list)))
    } else {
        Ok(ast.clone())
    }
}
