use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::MalVal;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    env.borrow_mut().set(
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

    env.borrow_mut().set(
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

    env.borrow_mut().set(
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

    env.borrow_mut().set(
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
}
