use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::MalVal;

pub fn register(env: Rc<RefCell<MalEnv>>) {
    env.borrow_mut().set(
        "list",
        &MalVal::Function(|args| Ok(MalVal::List(Rc::new(args.to_vec())))),
    );

    env.borrow_mut().set(
        "list?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::List(_)) => Ok(MalVal::Bool(true)),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    env.borrow_mut().set(
        "vector?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::Vector(_)) => Ok(MalVal::Bool(true)),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    env.borrow_mut().set(
        "empty?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::List(l)) | Some(MalVal::Vector(l)) => Ok(MalVal::Bool(l.is_empty())),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    env.borrow_mut().set(
        "count",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::Nil) => Ok(MalVal::Int(0)),
            Some(MalVal::List(l)) => Ok(MalVal::Int(l.len() as i64)),
            Some(MalVal::Vector(l)) => Ok(MalVal::Int(l.len() as i64)),
            _ => Ok(MalVal::Int(0)),
        }),
    );
}
