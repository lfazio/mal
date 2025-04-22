use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    builtin_register!(env, "apply", apply);
    builtin_register!(env, "atom", atom);
    builtin_register!(env, "atom?", is_atom);
    builtin_register!(env, "assoc", assoc);
    builtin_register!(env, "concat", concat);
    builtin_register!(env, "cons", cons);
    builtin_register!(env, "contains?", contains);
    builtin_register!(env, "count", count);
    builtin_register!(env, "deref", deref);
    builtin_register!(env, "dissoc", dissoc);
    builtin_register!(env, "empty?", is_empty);
    builtin_register!(env, "false?", is_false);
    builtin_register!(env, "first", first);
    builtin_register!(env, "get", get);
    builtin_register!(env, "hash-map", hash_map);
    builtin_register!(env, "keys", keys);
    builtin_register!(env, "keyword", keyword);
    builtin_register!(env, "keyword?", is_keyword);
    builtin_register!(env, "list", list);
    builtin_register!(env, "list?", is_list);
    builtin_register!(env, "macro?", is_macro);
    builtin_register!(env, "map", map);
    builtin_register!(env, "map?", is_map);
    builtin_register!(env, "nil?", is_nil);
    builtin_register!(env, "nth", nth);
    builtin_register!(env, "reset!", reset);
    builtin_register!(env, "rest", rest);
    builtin_register!(env, "sequential?", is_sequential);
    builtin_register!(env, "swap!", swap);
    builtin_register!(env, "symbol", symbol);
    builtin_register!(env, "symbol?", is_symbol);
    builtin_register!(env, "true?", is_true);
    builtin_register!(env, "vals", vals);
    builtin_register!(env, "vec", vec);
    builtin_register!(env, "vector", vector);
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

fn vector(args: &[MalVal]) -> MalReturn {
    Ok(MalVal::Vector(Rc::new(args.to_vec())))
}

fn is_vector(args: &[MalVal]) -> MalReturn {
    Ok(MalVal::Bool(
        args.first().unwrap_or(&MalVal::Bool(false)).is_vector(),
    ))
}

fn is_sequential(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error(
            "sequential? expects one argument".to_string(),
        ));
    }

    if args[0].is_list() || args[0].is_vector() {
        return Ok(MalVal::Bool(true));
    }

    Ok(MalVal::Bool(false))
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

fn is_nil(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("nil? expects one argument".to_string()));
    }

    Ok(MalVal::Bool(args[0].is_nil()))
}

fn is_true(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("false? expects one argument".to_string()));
    }

    if let MalVal::Bool(b) = args[0] {
        return Ok(MalVal::Bool(b));
    }

    Ok(MalVal::Bool(false))
}

fn is_false(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("false? expects one argument".to_string()));
    }

    if let MalVal::Bool(b) = args[0] {
        return Ok(MalVal::Bool(!b));
    }

    Ok(MalVal::Bool(false))
}

fn keyword(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("keyword expects one argument".to_string()));
    }

    if let MalVal::Str(s) = args[0].clone() {
        if s.starts_with(":") {
            return Ok(MalVal::Str(format!(
                "\u{29e}{}",
                String::from(s.strip_prefix(':').unwrap())
            )));
        } else if s.starts_with("\u{29e}") {
            return Ok(MalVal::Str(s));
        } else {
            return Ok(MalVal::Str(format!("\u{29e}{}", s)));
        }
    }

    Err(MalError::Error(
        "keyword expects a string argument".to_string(),
    ))
}

fn is_keyword(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("keyword? expects one argument".to_string()));
    }

    if let MalVal::Str(s) = args[0].clone() {
        if s.starts_with("\u{29e}") {
            return Ok(MalVal::Bool(true));
        }
    }

    Ok(MalVal::Bool(false))
}

fn symbol(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("symbol expects one argument".to_string()));
    }

    if let MalVal::Str(s) = args[0].clone() {
        return Ok(MalVal::Symbol(s));
    }

    Err(MalError::Error(
        "symbol expects a string argument".to_string(),
    ))
}

fn is_symbol(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("symbol? expects one argument".to_string()));
    }

    if let MalVal::Symbol(_) = args[0] {
        return Ok(MalVal::Bool(true));
    }

    Ok(MalVal::Bool(false))
}

fn hash_map(args: &[MalVal]) -> MalReturn {
    let mut map = HashMap::new();

    if args.len() % 2 != 0 {
        return Err(MalError::Error(
            "hashmap expects an even number of arguments".to_string(),
        ));
    }

    for idx in (0..args.len()).step_by(2) {
        if let (MalVal::Str(k), v) = (&args[idx], &args[idx + 1]) {
            map.insert(k.clone(), v.clone());
        } else {
            return Err(MalError::Error("hashmap expects string keys".to_string()));
        }
    }

    Ok(MalVal::Hashmap(Rc::new(map)))
}

fn is_map(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("is_map? expects one argument".to_string()));
    }

    match &args[0] {
        MalVal::Hashmap(_) => Ok(MalVal::Bool(true)),
        _ => Ok(MalVal::Bool(false)),
    }
}

fn assoc(args: &[MalVal]) -> MalReturn {
    if args.len() < 3 {
        return Err(MalError::Error(
            "assoc expects at least three arguments".to_string(),
        ));
    }

    if let MalVal::Hashmap(hm) = &args[0] {
        let mut new_map: HashMap<String, MalVal> = HashMap::new();
        for (k, v) in hm.iter() {
            new_map.insert(k.clone(), v.clone());
        }

        if args.len() % 2 != 1 {
            return Err(MalError::Error(
                "assoc expects an even number of arguments".to_string(),
            ));
        }

        for idx in (1..args.len()).step_by(2) {
            if let (MalVal::Str(k), v) = (&args[idx], &args[idx + 1]) {
                new_map.insert(k.clone(), v.clone());
            } else {
                return Err(MalError::Error("assoc expects string keys".to_string()));
            }
        }
        return Ok(MalVal::Hashmap(Rc::new(new_map)));
    }

    Err(MalError::Error(
        "assoc expects a hashmap as first argument".to_string(),
    ))
}

fn dissoc(args: &[MalVal]) -> MalReturn {
    if args.len() < 2 {
        return Err(MalError::Error(
            "dissoc expects at least two arguments".to_string(),
        ));
    }

    if let MalVal::Hashmap(hm) = &args[0] {
        let mut new_map: HashMap<String, MalVal> = HashMap::new();
        for (k, v) in hm.iter() {
            new_map.insert(k.clone(), v.clone());
        }

        for arg in &args[1..] {
            if let MalVal::Str(k) = arg {
                new_map.remove(k);
            } else {
                return Err(MalError::Error("dissoc expects string keys".to_string()));
            }
        }
        return Ok(MalVal::Hashmap(Rc::new(new_map)));
    }

    Err(MalError::Error(
        "dissoc expects a hashmap as first argument".to_string(),
    ))
}

fn get(args: &[MalVal]) -> MalReturn {
    if args.len() != 2 {
        return Err(MalError::Error("get expects two arguments".to_string()));
    }

    if let MalVal::Nil = args[0] {
        return Ok(MalVal::Nil);
    }

    if let MalVal::Hashmap(hm) = &args[0] {
        if let MalVal::Str(s) = &args[1] {
            return Ok(hm.get(s).cloned().unwrap_or(MalVal::Nil));
        }
    }

    Err(MalError::Error(
        "get expects a hashmap as first argument".to_string(),
    ))
}

fn contains(args: &[MalVal]) -> MalReturn {
    if args.len() != 2 {
        return Err(MalError::Error(
            "contains? expects two arguments".to_string(),
        ));
    }

    if let MalVal::Hashmap(h) = &args[0] {
        if let MalVal::Str(s) = &args[1] {
            return Ok(MalVal::Bool(h.contains_key(s)));
        }
    }

    Ok(MalVal::Bool(false))
}

fn keys(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("keys expects one argument".to_string()));
    }

    if let MalVal::Hashmap(hm) = &args[0] {
        return Ok(MalVal::List(Rc::new(
            hm.keys().map(|k| MalVal::Str(k.clone())).collect(),
        )));
    }

    Err(MalError::Error(
        "keys expects a hashmap as first argument".to_string(),
    ))
}

fn vals(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error("vals expects one argument".to_string()));
    }

    if let MalVal::Hashmap(hm) = &args[0] {
        return Ok(MalVal::List(Rc::new(hm.values().cloned().collect())));
    }

    Err(MalError::Error(
        "vals expects a hashmap as first argument".to_string(),
    ))
}

fn apply(args: &[MalVal]) -> MalReturn {
    if args.len() < 2 {
        return Err(MalError::Error(
            "apply expects at least two arguments".to_string(),
        ));
    }

    let func = &args[0];
    match &func {
        MalVal::Function(_) | MalVal::Lambda(_) => {
            let mut new_args: Vec<MalVal> = vec![];

            for arg in args.iter().skip(1) {
                if let MalVal::List(seq) = arg {
                    new_args.extend(seq.iter().cloned());
                } else if let MalVal::Vector(seq) = arg {
                    new_args.extend(seq.iter().cloned());
                } else {
                    new_args.push(arg.clone());
                }
            }

            func.apply(new_args.to_vec().as_slice())
        }
        _ => Ok(args[0].clone()),
    }
}

fn map(args: &[MalVal]) -> MalReturn {
    if args.len() < 2 {
        return Err(MalError::Error(
            "apply expects at least two arguments".to_string(),
        ));
    }

    let mut new_args: Vec<MalVal> = vec![];
    match &args[0] {
        MalVal::Lambda(func) => {
            for arg in args.iter().skip(1) {
                if let MalVal::List(seq) = arg {
                    let mapped: Vec<MalVal> = seq
                        .iter()
                        .map(|x| func.apply(&[x.clone()]))
                        .collect::<Result<Vec<_>, _>>()?;
                    new_args.push(MalVal::List(Rc::new(mapped)));
                } else if let MalVal::Vector(seq) = arg {
                    let mapped: Vec<MalVal> = seq
                        .iter()
                        .map(|x| func.apply(&[x.clone()]))
                        .collect::<Result<Vec<_>, _>>()?;
                    new_args.push(MalVal::List(Rc::new(mapped)));
                }
            }
        }
        MalVal::Function(func) => {
            for arg in args.iter().skip(1) {
                if let MalVal::List(seq) = arg {
                    let mapped: Vec<MalVal> = seq
                        .iter()
                        .map(|x| func(&[x.clone()]))
                        .collect::<Result<Vec<_>, _>>()?;
                    new_args.push(MalVal::List(Rc::new(mapped)));
                } else if let MalVal::Vector(seq) = arg {
                    let mapped: Vec<MalVal> = seq
                        .iter()
                        .map(|x| func(&[x.clone()]))
                        .collect::<Result<Vec<_>, _>>()?;
                    new_args.push(MalVal::List(Rc::new(mapped)));
                }
            }
        }
        _ => return Ok(args[0].clone()),
    }

    Ok(new_args.last().unwrap_or(&MalVal::Nil).clone())
}
