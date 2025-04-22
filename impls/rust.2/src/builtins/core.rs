use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    builtin_register!(env, "atom", atom);
    builtin_register!(env, "atom?", is_atom);
    builtin_register!(env, "concat", concat);
    builtin_register!(env, "cons", cons);
    builtin_register!(env, "count", count);
    builtin_register!(env, "deref", deref);
    builtin_register!(env, "empty?", is_empty);
    builtin_register!(env, "first", first);
    builtin_register!(env, "list", list);
    builtin_register!(env, "list?", is_list);
    builtin_register!(env, "macro?", is_macro);
    builtin_register!(env, "nth", nth);
    builtin_register!(env, "reset!", reset);
    builtin_register!(env, "rest", rest);
    builtin_register!(env, "swap!", swap);
    builtin_register!(env, "vec", vec);
    builtin_register!(env, "vector?", is_vector);
}

fn list(args: &[MalVal]) -> MalReturn {
    Ok(MalVal::List(Rc::new(args.to_vec())))
}

fn is_list(args: &[MalVal]) -> MalReturn {
    Ok(MalVal::Bool(
        args.first().unwrap_or(&MalVal::Bool(false)).is_list(),
    ))
}

fn is_vector(args: &[MalVal]) -> MalReturn {
    Ok(MalVal::Bool(
        args.first().unwrap_or(&MalVal::Bool(false)).is_vector(),
    ))
}

fn is_empty(args: &[MalVal]) -> MalReturn {
    match args.first() {
        Some(MalVal::List(l)) | Some(MalVal::Vector(l)) => Ok(MalVal::Bool(l.is_empty())),
        _ => Ok(MalVal::Bool(false)),
    }
}

fn count(args: &[MalVal]) -> MalReturn {
    match args.first() {
        Some(MalVal::List(l)) => Ok(MalVal::Int(l.len() as i64)),
        Some(MalVal::Vector(l)) => Ok(MalVal::Int(l.len() as i64)),
        _ => Ok(MalVal::Int(0)),
    }
}

fn is_atom(args: &[MalVal]) -> MalReturn {
    match args.first() {
        Some(MalVal::Atom(_)) => Ok(MalVal::Bool(true)),
        _ => Ok(MalVal::Bool(false)),
    }
}

fn atom(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("atom expects one argument".to_string()));
    }
    let val = args[0].clone();

    Ok(MalVal::Atom(Rc::new(RefCell::new(val))))
}

fn deref(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("deref expects one argument".to_string()));
    }

    if let Some(MalVal::Atom(a)) = args.first() {
        return Ok(a.borrow().clone());
    }

    Err(MalError::Error("deref expects an atom".to_string()))
}

fn reset(args: &[MalVal]) -> MalReturn {
    if args.len() != 2 {
        return Err(MalError::Error("reset! expects two arguments".to_string()));
    }

    if let Some(MalVal::Atom(a)) = args.first() {
        let val = args[1].clone();
        a.replace(val.clone());
        return Ok(val);
    }

    Err(MalError::Error("reset! expects an atom".to_string()))
}

fn swap(args: &[MalVal]) -> MalReturn {
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
        return Ok(new_val);
    }

    Err(MalError::Error("swap! expects an atom".to_string()))
}

fn cons(args: &[MalVal]) -> MalReturn {
    if args.len() != 2 {
        return Err(MalError::Error("cons expects two arguments".to_string()));
    }

    if let MalVal::List(seq) = &args[1] {
        return Ok(MalVal::List(Rc::new(
            vec![args[0].clone()]
                .into_iter()
                .chain(seq.iter().cloned())
                .collect(),
        )));
    } else if let MalVal::Vector(seq) = &args[1] {
        return Ok(MalVal::List(Rc::new(
            vec![args[0].clone()]
                .into_iter()
                .chain(seq.iter().cloned())
                .collect(),
        )));
    }

    Err(MalError::Error(
        "cons expects a list or a vector as second argument".to_string(),
    ))
}

fn concat(args: &[MalVal]) -> MalReturn {
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
}

fn vec(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("vec expects one argument".to_string()));
    }

    if let MalVal::List(l) = &args[0] {
        let vec: Vec<MalVal> = l.iter().cloned().collect();
        return Ok(MalVal::Vector(Rc::new(vec)));
    }

    if let MalVal::Vector(_) = &args[0] {
        return Ok(args[0].clone());
    }

    Err(MalError::Error(
        "vec expects a list as argument".to_string(),
    ))
}

fn nth(args: &[MalVal]) -> MalReturn {
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
}

fn first(args: &[MalVal]) -> MalReturn {
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
}

fn rest(args: &[MalVal]) -> MalReturn {
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
}

fn is_macro(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("macro? expects one argument".to_string()));
    }

    if let MalVal::Lambda(l) = &args[0] {
        return Ok(MalVal::Bool(l.is_macro()));
    }

    Ok(MalVal::Bool(false))
}
