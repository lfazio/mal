use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::MalEnv;
use crate::error::MalError;
use crate::types::MalVal;

pub fn eval(input: Result<MalVal, MalError>, env: Rc<RefCell<MalEnv>>) -> Result<MalVal, MalError> {
    let ast = input?;
    let mut debug_eval = false;

    match env.borrow().get("DEBUG-EVAL") {
        Ok(MalVal::Bool(debug)) => debug_eval = debug,
        Ok(MalVal::Int(_)) => debug_eval = true,
        Ok(MalVal::Str(_)) => debug_eval = true,
        Ok(MalVal::List(_)) => debug_eval = true,
        Ok(_) => (),
        Err(_) => (),
    }
    if debug_eval {
        println!("EVAL: {}", &ast);
    }

    match ast.clone() {
        MalVal::Symbol(k) => env.borrow().get(&k),
        MalVal::Vector(mut v) => {
            for v in v.iter_mut() {
                *v = eval(Ok(v.clone()), env.clone())?;
            }

            Ok(MalVal::Vector(v))
        }
        MalVal::List(mut l) => {
            if l.is_empty() {
                return Ok(ast);
            }

            if let MalVal::Symbol(s) = &l[0] {
                match s.as_str() {
                    "def!" => {
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), env.clone())?;
                        env.borrow_mut().set(&key.to_string(), val.clone());
                        return Ok(val);
                    }
                    "let*" => {
                        let outer = env;
                        let new_env = MalEnv::new(Some(outer));
                        let new_env = Rc::new(RefCell::new(new_env));
                        let bindings = l[1].clone();
                        let body = l[2].clone();

                        match bindings {
                            MalVal::List(bindings) | MalVal::Vector(bindings) => {
                                for i in (0..bindings.len()).step_by(2) {
                                    let key = bindings[i].clone();
                                    let val = eval(Ok(bindings[i + 1].clone()), new_env.clone())?;
                                    new_env.borrow_mut().set(&key.to_string(), val);
                                }
                            }
                            _ => {
                                return Err(MalError::Error(
                                    "let* bindings must be a list or vector".to_string(),
                                ))
                            }
                        }

                        return eval(Ok(body), new_env);
                    }
                    _ => (),
                }
            }

            for e in l.iter_mut() {
                match eval(Ok(e.clone()), env.clone()) {
                    Ok(v) => *e = v,
                    Err(e) => return Err(e),
                }
            }

            let func = l.remove(0);
            match func.apply(l) {
                Ok(v) => Ok(v),
                Err(e) => Err(MalError::Error(e)),
            }
        }
        MalVal::Hashmap(mut h) => {
            let entries: Vec<_> = h.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            for (k, v) in entries {
                h.entry(k)
                    .and_modify(|val| *val = eval(Ok(v), env.clone()).unwrap());
            }

            Ok(MalVal::Hashmap(h))
        }
        _ => Ok(ast),
    }
}
