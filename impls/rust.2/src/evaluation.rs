use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::environment::{MalEnv, get_env_repl};
use crate::types::error::MalError;
use crate::types::lambda::Lambda;
use crate::types::{MalReturn, MalVal};

pub fn eval(input: MalReturn, env: &Rc<RefCell<MalEnv>>) -> MalReturn {
    let mut ast = input?;
    let mut env = env;
    let mut new_env: Rc<RefCell<MalEnv>>;
    let mut repl_env;

    loop {
        // TCO loop
        let debug_eval = match env.borrow().get("DEBUG-EVAL") {
            Ok(MalVal::Bool(debug)) => debug,
            Ok(MalVal::Int(_)) | Ok(MalVal::Str(_)) | Ok(MalVal::List(_)) => true,
            _ => false,
        };

        if debug_eval {
            println!("EVAL: {}", &ast.pr_str(true));
        }

        match ast {
            MalVal::Symbol(k) => return env.borrow().get(&k),
            MalVal::Vector(v) => {
                let mut s: Vec<MalVal> = vec![];
                for v in v.iter() {
                    s.push(eval(Ok(v.clone()), env)?);
                }

                return Ok(MalVal::Vector(Rc::new(s)));
            }
            MalVal::List(l) => {
                if l.is_empty() {
                    return Ok(MalVal::List(Rc::new(vec![])));
                }

                match &l[0] {
                    MalVal::Symbol(s) if s == "def!" => {
                        repl_env = get_env_repl(env);
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), &repl_env)?;
                        repl_env.borrow_mut().set(&key.pr_str(true), &val);
                        return Ok(val);
                    }
                    MalVal::Symbol(s) if s == "let*" => {
                        let bindings = l[1].clone();
                        let body = l[2].clone();

                        match bindings {
                            MalVal::List(bindings) | MalVal::Vector(bindings) => {
                                new_env = Rc::new(RefCell::new(MalEnv::new(Some(Rc::clone(env)))));
                                for i in (0..bindings.len()).step_by(2) {
                                    let key = bindings[i].clone();
                                    let val = eval(Ok(bindings[i + 1].clone()), &new_env)?;
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
                        env = &new_env;
                        continue;
                    }
                    MalVal::Symbol(s) if s == "do" => {
                        for e in l.iter().take(l.len() - 1).skip(1) {
                            _ = eval(Ok(e.clone()), env)?;
                        }
                        ast = l.last().unwrap_or(&MalVal::Nil).clone();
                        continue;
                    }
                    MalVal::Symbol(s) if s == "if" => match eval(Ok(l[1].clone()), env)? {
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
                            Rc::clone(env),
                        ))));
                    }
                    MalVal::Symbol(s) if s == "eval" => {
                        let args = l[1].clone();
                        ast = eval(Ok(args), env)?;
                        repl_env = get_env_repl(env);
                        env = &repl_env;
                        continue;
                    }

                    _ => match eval(Ok(l[0].clone()), env) {
                        Ok(func @ MalVal::Function(_)) => {
                            let argv: Vec<MalVal> = l
                                .iter()
                                .skip(1)
                                .map(|x| eval(Ok(x.clone()), env))
                                .collect::<Result<Vec<_>, _>>()?;

                            return func.apply(&argv);
                        }
                        Ok(MalVal::Lambda(f)) => {
                            let argv: Vec<MalVal> = l
                                .iter()
                                .skip(1)
                                .map(|x| eval(Ok(x.clone()), env))
                                .collect::<Result<Vec<_>, _>>()?;

                            new_env = Rc::new(RefCell::new(f.bind(&argv)));
                            env = &new_env;
                            ast = (*f.ast).clone();
                            continue;
                        }
                        Ok(_) => {
                            return Err(MalError::Error("try to call a non-function".to_string()));
                        }
                        Err(e) => return Err(e),
                    },
                }
            }
            MalVal::Hashmap(h) => {
                let mut new_hm: HashMap<String, MalVal> = HashMap::new();
                let entries: Vec<_> = h.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                for (k, v) in entries {
                    new_hm.entry(k).insert_entry(eval(Ok(v), env).unwrap());
                }

                return Ok(MalVal::Hashmap(Rc::new(new_hm)));
            }
            _ => return Ok(ast),
        }
    } // TCO loop
}
