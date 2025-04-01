use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::lambda::Lambda;
use crate::types::{MalReturn, MalVal};

pub fn eval(input: MalReturn, env: Rc<RefCell<MalEnv>>) -> MalReturn {
    let mut ast = input?;
    let mut env = env;

    loop {
        // TCO loop
        let debug_eval = match env.clone().borrow().get("DEBUG-EVAL") {
            Ok(MalVal::Bool(debug)) => debug,
            Ok(MalVal::Int(_)) | Ok(MalVal::Str(_)) | Ok(MalVal::List(_)) => true,
            _ => false,
        };

        if debug_eval {
            println!("EVAL: {}", &ast.pr_str(true));
        }

        match ast.clone() {
            MalVal::Symbol(k) => return env.borrow().get(&k),
            MalVal::Vector(v) => {
                let mut s: Vec<MalVal> = vec![];
                for v in v.iter() {
                    s.push(eval(Ok(v.clone()), env.clone())?);
                }

                return Ok(MalVal::Vector(Rc::new(s)));
            }
            MalVal::List(l) => {
                if l.is_empty() {
                    return Ok(ast);
                }

                match &l[0] {
                    MalVal::Symbol(s) if s == "def!" => {
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), env.clone())?;
                        env.borrow_mut().set(&key.pr_str(true), &val);
                        return Ok(val);
                    }
                    MalVal::Symbol(s) if s == "let*" => {
                        let outer = env.clone();
                        let new_env = MalEnv::new(Some(outer));
                        let new_env = Rc::new(RefCell::new(new_env));
                        let bindings = l[1].clone();
                        let body = l[2].clone();

                        match bindings {
                            MalVal::List(bindings) | MalVal::Vector(bindings) => {
                                for i in (0..bindings.len()).step_by(2) {
                                    let key = bindings[i].clone();
                                    let val = eval(Ok(bindings[i + 1].clone()), new_env.clone())?;
                                    new_env.borrow_mut().set(&key.pr_str(true), &val);
                                }
                            }
                            _ => {
                                return Err(MalError::Error(
                                    "let* bindings must be a list or vector".to_string(),
                                ));
                            }
                        }

                        ast = body;
                        env = new_env;
                        continue;
                    }
                    MalVal::Symbol(s) if s == "do" => {
                        let mut s = vec![];
                        let len = if l.len() >= 2 { l.len() } else { 0 };
                        for e in l.iter().skip(1).take(len) {
                            s.push(eval(Ok(e.clone()), env.clone())?);
                        }
                        if len >= 2 {
                            ast = l.last().unwrap().clone();
                            continue;
                        }
                    }
                    MalVal::Symbol(s) if s == "if" => match eval(Ok(l[1].clone()), env.clone())? {
                        MalVal::Bool(false) | MalVal::Nil => {
                            if l.len() == 4 {
                                ast = l[3].clone();
                                continue;
                            } else {
                                return Ok(MalVal::Nil);
                            }
                        }
                        _ => {
                            ast = l[2].clone();
                            continue;
                        }
                    },
                    MalVal::Symbol(s) if s == "fn*" => {
                        let args = l[1].clone();

                        return Ok(MalVal::Lambda(Rc::new(Lambda::new(
                            eval,
                            l[2].clone(),
                            args,
                            env.clone(),
                        ))));
                    }
                    _ => match eval(Ok(l[0].clone()), env.clone()) {
                        Ok(func @ MalVal::Function(_)) => {
                            let argv: Vec<MalVal> = l
                                .iter()
                                .skip(1)
                                .map(|x| eval(Ok(x.clone()), env.clone()))
                                .collect::<Result<Vec<_>, _>>()?;

                            return func.apply(argv);
                        }
                        Ok(MalVal::Lambda(f)) => {
                            let argv: Vec<MalVal> = l
                                .iter()
                                .skip(1)
                                .map(|x| eval(Ok(x.clone()), env.clone()))
                                .collect::<Result<Vec<_>, _>>()?;

                            env = Rc::new(RefCell::new(f.bind(argv)));
                            ast = f.ast.clone();
                            continue;
                        }
                        Ok(_) => {
                            return Err(MalError::Error("try to call a non-function".to_string()))
                        }
                        Err(e) => return Err(e),
                    },
                }
            }
            MalVal::Hashmap(h) => {
                let mut new_hm: HashMap<String, MalVal> = HashMap::new();
                let entries: Vec<_> = h.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                for (k, v) in entries {
                    new_hm
                        .entry(k)
                        .insert_entry(eval(Ok(v), env.clone()).unwrap());
                }

                return Ok(MalVal::Hashmap(Rc::new(new_hm)));
            }
            _ => return Ok(ast),
        }
    } // TCO loop
}
