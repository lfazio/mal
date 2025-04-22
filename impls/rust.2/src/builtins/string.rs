use std::io::Write;
use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

use crate::reader;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    builtin_register!(env, "pr-str", |args| {
        Ok(MalVal::Str(
            args.iter()
                .map(|x| x.pr_str(true))
                .collect::<Vec<_>>()
                .join(" "),
        ))
    });

    builtin_register!(env, "str", |args| {
        Ok(MalVal::Str(
            args.iter()
                .map(|x| x.pr_str(false))
                .collect::<Vec<_>>()
                .join(""),
        ))
    });

    builtin_register!(env, "prn", |args| {
        let txt = args
            .iter()
            .map(|x| x.pr_str(true))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", txt);

        Ok(MalVal::Nil)
    });

    builtin_register!(env, "println", |args| {
        let txt = args
            .iter()
            .map(|x| x.pr_str(false))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", txt);

        Ok(MalVal::Nil)
    });

    builtin_register!(env, "read-string", |args| {
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
    });

    builtin_register!(env, "readline", readline);
}

fn readline(args: &[MalVal]) -> MalReturn {
    if args.len() > 1 {
        return Err(MalError::Error(
            "readline expects only one string".to_string(),
        ));
    }
    if let Some(MalVal::Str(prompt)) = args.first() {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        Ok(MalVal::Str(input.trim_end().to_string()))
    } else {
        Err(MalError::Error(
            "readline expects one argument of string type".to_string(),
        ))
    }
}
