use std::{cell::RefCell, rc::Rc};

use crate::types::MalVal;
use crate::types::environment::MalEnv;

fn fn_op(func: &str, args: &[i64]) -> i64 {
    match func {
        "+" => args.iter().sum(),
        "-" => args[1..].iter().fold(args[0], |acc, x| acc - x),
        "*" => args.iter().product(),
        "/" => args[1..].iter().fold(args[0], |acc, x| acc / x),
        _ => panic!("Unknown operator: {}", func),
    }
}

fn fn_cmp(op: &str, args: &[MalVal]) -> bool {
    match op {
        "=" => args.windows(2).all(|w| w[0] == w[1]),
        "!=" => args.windows(2).all(|w| w[0] != w[1]),
        "<" => args.windows(2).all(|w| w[0] < w[1]),
        "<=" => args.windows(2).all(|w| w[0] <= w[1]),
        ">" => args.windows(2).all(|w| w[0] > w[1]),
        ">=" => args.windows(2).all(|w| w[0] >= w[1]),
        _ => panic!("Unknown operator: {}", op),
    }
}

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    let _ = env.borrow_mut().set(
        "+",
        &MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "+",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    let _ = env.borrow_mut().set(
        "-",
        &MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "-",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    let _ = env.borrow_mut().set(
        "*",
        &MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "*",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    let _ = env.borrow_mut().set(
        "/",
        &MalVal::Function(|args| {
            Ok(MalVal::Int(fn_op(
                "/",
                &args
                    .iter()
                    .map(|x| if let MalVal::Int(v) = x { *v } else { 0 })
                    .collect::<Vec<_>>(),
            )))
        }),
    );

    let _ = env.borrow_mut().set(
        "=",
        &MalVal::Function(|args| Ok(MalVal::Bool(fn_cmp("=", args)))),
    );

    let _ = env.borrow_mut().set(
        "<",
        &MalVal::Function(|args| Ok(MalVal::Bool(fn_cmp("<", args)))),
    );

    let _ = env.borrow_mut().set(
        "<=",
        &MalVal::Function(|args| Ok(MalVal::Bool(fn_cmp("<=", args)))),
    );

    let _ = env.borrow_mut().set(
        ">",
        &MalVal::Function(|args| Ok(MalVal::Bool(fn_cmp(">", args)))),
    );

    let _ = env.borrow_mut().set(
        ">=",
        &MalVal::Function(|args| Ok(MalVal::Bool(fn_cmp(">=", args)))),
    );
}
