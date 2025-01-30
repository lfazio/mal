use std::cell::RefCell;
use std::rc::Rc;

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::lambda::Lambda;
use crate::types::{MalReturn, MalVal};

pub fn eval(input: MalReturn, env: Rc<RefCell<MalEnv>>) -> MalReturn {
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
        println!("EVAL: {}", &ast.pr_str(true));
    }

    match ast.clone() {
        MalVal::Symbol(k) => env.borrow().get(&k),
        MalVal::Vector(mut v) => {
            for v in v.iter_mut() {
                *v = eval(Ok(v.clone()), env.clone())?;
            }

            Ok(MalVal::Vector(v))
        }
        MalVal::List(l) => {
            if l.is_empty() {
                return Ok(ast);
            }

            if let MalVal::Symbol(s) = &l[0] {
                match s.as_str() {
                    "def!" => {
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), env.clone())?;
                        env.borrow_mut().set(&key.pr_str(true), val.clone());
                        return Ok(val);
                    }
                    "let*" => {
                        let outer = env;
                        let new_env = MalEnv::new(Some(outer));
                        let new_env = Rc::new(RefCell::new(new_env));
                        let bindings = l[1].clone();
                        let body = l[2].clone();

                        match bindings {
                            MalVal::List(bindings)
                            | MalVal::Vector(bindings) => {
                                for i in (0..bindings.len()).step_by(2) {
                                    let key = bindings[i].clone();
                                    let val = eval(Ok(bindings[i + 1].clone()), new_env.clone())?;
                                    new_env.borrow_mut().set(&key.pr_str(true), val);
                                }
                            }
                            _ => {
                                return Err(MalError::Error(
                                    "let* bindings must be a list or vector".to_string(),
                                ));
                            }
                        }

                        return eval(Ok(body), new_env);
                    }
                    "do" => {
                        let mut result = MalVal::Nil;
                        for e in l.iter().skip(1) {
                            result = eval(Ok(e.clone()), env.clone())?;
                        }
                        return Ok(result);
                    }
                    "if" => {
                        let mut result = MalVal::Nil;
                        match eval(Ok(l[1].clone()), env.clone())? {
                            MalVal::Bool(false)
                            | MalVal::Nil => {
                                if l.len() == 4 {
                                    result = eval(Ok(l[3].clone()), env.clone())?;
                                }
                            }
                            _ => {
                                result = eval(Ok(l[2].clone()), env.clone())?;
                            }
                        }
                        return Ok(result);
                    }
                    "fn*" | "lambda" => {
                        return Ok(MalVal::Lambda(Rc::new(RefCell::new(Lambda::new(
                            eval,
                            l[2].clone(),
                            l[1].clone(),
                            env.clone(),
                        )))));
                    }
                    _ => (),
                }
            }

            let argv: Vec<MalVal> = l.iter().map(|x| eval(Ok(x.clone()), Rc::clone(&env)).unwrap()).collect();
            let func = &argv[0];

            if let MalVal::Function(_) = func {
                return func.apply(argv[1..].to_vec());
            }
            if let MalVal::Lambda(_) = func {
                return func.apply(argv[1..].to_vec());
            }
            Ok(MalVal::List(argv))
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
