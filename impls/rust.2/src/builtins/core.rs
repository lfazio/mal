use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::MalVal;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    let _ = env.borrow_mut().set(
        "list",
        &MalVal::Function(|args| Ok(MalVal::List(Rc::new(args.to_vec())))),
    );

    let _ = env.borrow_mut().set(
        "list?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::List(_)) => Ok(MalVal::Bool(true)),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    let _ = env.borrow_mut().set(
        "vector?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::Vector(_)) => Ok(MalVal::Bool(true)),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    let _ = env.borrow_mut().set(
        "empty?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::List(l)) | Some(MalVal::Vector(l)) => Ok(MalVal::Bool(l.is_empty())),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    let _ = env.borrow_mut().set(
        "count",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::Nil) => Ok(MalVal::Int(0)),
            Some(MalVal::List(l)) => Ok(MalVal::Int(l.len() as i64)),
            Some(MalVal::Vector(l)) => Ok(MalVal::Int(l.len() as i64)),
            _ => Ok(MalVal::Int(0)),
        }),
    );

    let _ = env.borrow_mut().set(
        "atom?",
        &MalVal::Function(|args| match args.first() {
            Some(MalVal::Atom(_)) => Ok(MalVal::Bool(true)),
            _ => Ok(MalVal::Bool(false)),
        }),
    );

    let _ = env.borrow_mut().set(
        "atom",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("atom expects one argument".to_string()));
            }
            let val = args[0].clone();
            Ok(MalVal::Atom(Rc::new(RefCell::new(val))))
        }),
    );

    let _ = env.borrow_mut().set(
        "deref",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("deref expects one argument".to_string()));
            }

            if let Some(MalVal::Atom(a)) = args.first() {
                Ok(a.borrow().clone())
            } else {
                Err(MalError::Error("deref expects an atom".to_string()))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "reset!",
        &MalVal::Function(|args| {
            if args.len() != 2 {
                return Err(MalError::Error("reset! expects two arguments".to_string()));
            }
            if let Some(MalVal::Atom(a)) = args.first() {
                let val = args[1].clone();
                a.replace(val.clone());
                Ok(val)
            } else {
                Err(MalError::Error("reset! expects an atom".to_string()))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "swap!",
        &MalVal::Function(|args| {
            if args.len() < 2 {
                return Err(MalError::Error(
                    "swap! expects at least two arguments".to_string(),
                ));
            }
            if let Some(MalVal::Atom(a)) = args.first() {
                let f = args[1].clone();
                let mut arg = args[2..].to_vec();
                arg.insert(0, a.borrow().clone());
                let new_val = f.apply(&arg)?;
                *a.borrow_mut() = new_val.clone();
                Ok(new_val)
            } else {
                Err(MalError::Error("swap! expects an atom".to_string()))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "cons",
        &MalVal::Function(|args| {
            if let MalVal::List(seq) = &args[1] {
                Ok(MalVal::List(Rc::new(
                    vec![args[0].clone()]
                        .into_iter()
                        .chain(seq.iter().cloned())
                        .collect(),
                )))
            } else if let MalVal::Vector(seq) = &args[1] {
                Ok(MalVal::List(Rc::new(
                    vec![args[0].clone()]
                        .into_iter()
                        .chain(seq.iter().cloned())
                        .collect(),
                )))
            } else {
                Err(MalError::Error(
                    "cons expects a list or a vector as second argument".to_string(),
                ))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "concat",
        &MalVal::Function(|args| {
            let mut concatenated = vec![];

            for arg in args.iter() {
                if let MalVal::List(seq) = arg {
                    concatenated.extend(seq.iter().cloned());
                } else if let MalVal::Vector(seq) = arg {
                    concatenated.extend(seq.iter().cloned());
                } else {
                    return Err(MalError::Error(
                        "concat expects lists as arguments".to_string(),
                    ));
                }
            }

            Ok(MalVal::List(Rc::new(concatenated)))
        }),
    );

    let _ = env.borrow_mut().set(
        "vec",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("vec expects one argument".to_string()));
            }
            if let MalVal::List(l) = &args[0] {
                let vec: Vec<MalVal> = l.iter().cloned().collect();

                Ok(MalVal::Vector(Rc::new(vec)))
            } else if let MalVal::Vector(_) = &args[0] {
                Ok(args[0].clone())
            } else {
                Err(MalError::Error(
                    "vec expects a list or a vector".to_string(),
                ))
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "nth",
        &MalVal::Function(|args| {
            if args.len() != 2 {
                return Err(MalError::Error("nth expects 2 arguments".to_string()));
            }

            let index = match &args[1] {
                MalVal::Int(i) => *i,
                _ => return Err(MalError::Error("nth expects an integer index".to_string())),
            };

            match &args[0] {
                MalVal::List(l) => {
                    if index < 0 || index >= l.len() as i64 {
                        return Err(MalError::Error("index out of bounds".to_string()));
                    }
                    Ok(l[index as usize].clone())
                }
                MalVal::Vector(v) => {
                    if index < 0 || index >= v.len() as i64 {
                        return Err(MalError::Error("index out of bounds".to_string()));
                    }

                    Ok(v[index as usize].clone())
                }
                MalVal::Nil => Ok(MalVal::Nil),
                _ => Err(MalError::Error(
                    "nth expects a list or a vector".to_string(),
                )),
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "first",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("first expects 1 arguments".to_string()));
            }

            match &args[0] {
                MalVal::List(l) => {
                    if l.is_empty() || l[0].is_nil() {
                        return Ok(MalVal::Nil);
                    }
                    Ok(l[0].clone())
                }
                MalVal::Vector(v) => {
                    if v.is_empty() || v[0].is_nil() {
                        return Ok(MalVal::Nil);
                    }
                    Ok(v[0].clone())
                }
                MalVal::Nil => Ok(MalVal::Nil),
                _ => Err(MalError::Error(
                    "first expects a list or a vector".to_string(),
                )),
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "rest",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("rest expects 1 arguments".to_string()));
            }

            match &args[0] {
                MalVal::List(l) => {
                    if l.is_empty() || l[0].is_nil() {
                        return Ok(MalVal::List(Rc::new(vec![])));
                    }
                    Ok(MalVal::List(Rc::new(l[1..].to_vec())))
                }
                MalVal::Vector(v) => {
                    if v.is_empty() || v[0].is_nil() {
                        return Ok(MalVal::List(Rc::new(vec![])));
                    }
                    Ok(MalVal::List(Rc::new(v[1..].to_vec())))
                }
                MalVal::Nil => Ok(MalVal::List(Rc::new(vec![]))),
                _ => Err(MalError::Error(
                    "rest expects a list or a vector".to_string(),
                )),
            }
        }),
    );

    let _ = env.borrow_mut().set(
        "macro?",
        &MalVal::Function(|args| {
            if args.len() != 1 {
                return Err(MalError::Error("macro? expects 1 arguments".to_string()));
            }

            match &args[0] {
                MalVal::Lambda(l) => Ok(MalVal::Bool(l.is_macro())),
                _ => Ok(MalVal::Bool(false)),
            }
        }),
    );
}
