use std::{cell::RefCell, rc::Rc};

use std::fs;

use crate::types::MalVal;
use crate::types::environment::MalEnv;
use crate::types::error::MalError;

use crate::reader;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    let _ = env.borrow_mut().set(
        "pr-str",
        &MalVal::Function(|args| {
            Ok(MalVal::Str(
                args.iter()
                    .map(|x| x.pr_str(true))
                    .collect::<Vec<_>>()
                    .join(" "),
            ))
        }),
    );

    let _ = env.borrow_mut().set(
        "str",
        &MalVal::Function(|args| {
            Ok(MalVal::Str(
                args.iter()
                    .map(|x| x.pr_str(false))
                    .collect::<Vec<_>>()
                    .join(""),
            ))
        }),
    );

    let _ = env.borrow_mut().set(
        "prn",
        &MalVal::Function(|args| {
            let txt = args
                .iter()
                .map(|x| x.pr_str(true))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", txt);
            Ok(MalVal::Nil)
        }),
    );

    let _ = env.borrow_mut().set(
        "println",
        &MalVal::Function(|args| {
            let txt = args
                .iter()
                .map(|x| x.pr_str(false))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", txt);
            Ok(MalVal::Nil)
        }),
    );

    let _ = env.borrow_mut().set(
        "read-string",
        &MalVal::Function(|args| {
            if args.len() > 1 {
                return Err(MalError::Error(
                    "read-string expects only one string".to_string(),
                ));
            }

            if let Some(MalVal::Str(s)) = args.first() {
                Ok(reader::read_str(s)?)
            } else {
                Err(MalError::Error(
                    "read-string expects one argument of string type".to_string(),
                ))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "slurp",
        &MalVal::Function(|args| {
            if args.len() > 1 {
                return Err(MalError::Error(
                    "read-string expects only one string".to_string(),
                ));
            }

            if let Some(MalVal::Str(s)) = args.first() {
                let contents =
                    fs::read_to_string(s).expect("Should have been able to read the file");
                Ok(MalVal::Str(contents))
            } else {
                Err(MalError::Error(
                    "slurp expects one argument of string type".to_string(),
                ))
            }
        }),
    );
}
