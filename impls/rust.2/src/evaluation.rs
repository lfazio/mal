use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::environment::{MalEnv, get_env_repl};
use crate::types::error::MalError;
use crate::types::lambda::Lambda;
use crate::types::{MalReturn, MalVal};

use crate::builtins;

use crate::hashmap;
use crate::lambda;
use crate::list;
use crate::new_env;
use crate::vector;

pub fn eval(input: MalReturn, env: &Rc<RefCell<MalEnv>>) -> MalReturn {
    let mut ast = input?;
    let mut env = env;
    let mut new_env: Rc<RefCell<MalEnv>>;
    let mut repl_env;

    loop {
        // TCO loop
        let debug_eval = match env.borrow().get("DEBUG-EVAL") {
            Ok(MalVal::Bool(debug)) => debug,
            Ok(MalVal::Int(_)) | Ok(MalVal::Str(_)) | Ok(MalVal::List(_, _)) => true,
            _ => false,
        };

        if debug_eval {
            println!("EVAL: {}", &ast.pr_str(true));
        }

        match &ast {
            MalVal::Symbol(k) => return env.borrow().get(k),
            MalVal::Vector(v, _) => {
                let mut s: Vec<MalVal> = vec![];
                for v in v.iter() {
                    s.push(eval(Ok(v.clone()), env)?);
                }

                return Ok(vector!(s));
            }
            MalVal::List(l, _) => {
                if l.is_empty() {
                    return Ok(list!());
                }

                match &l[0] {
                    MalVal::Symbol(s) if s == "def!" => {
                        repl_env = get_env_repl(env);
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), &repl_env)?;
                        let _ = repl_env.borrow_mut().set(&key.pr_str(true), &val);
                        return Ok(val);
                    }
                    MalVal::Symbol(s) if s == "defmacro!" => {
                        let key = l[1].clone();
                        let val = eval(Ok(l[2].clone()), env)?;

                        match &val {
                            MalVal::Lambda(l, _) => {
                                let args = l.get_args();
                                return env.borrow_mut().set(
                                    &key.pr_str(true),
                                    &lambda!(Lambda::new_macro(
                                        eval,
                                        l.get_ast(),
                                        list!(args),
                                        Rc::clone(env),
                                    )),
                                );
                            }
                            _ => {
                                return Err(MalError::Error(
                                    "defmacro! expects a lambda".to_string(),
                                ));
                            }
                        }
                    }
                    MalVal::Symbol(s) if s == "let*" => {
                        let bindings = l[1].clone();
                        let body = l[2].clone();

                        match bindings {
                            MalVal::List(bindings, _) | MalVal::Vector(bindings, _) => {
                                new_env = new_env!(Some(Rc::clone(env)));
                                for i in (0..bindings.len()).step_by(2) {
                                    let key = bindings[i].clone();
                                    let val = eval(Ok(bindings[i + 1].clone()), &new_env)?;
                                    let _ = new_env.borrow_mut().set(&key.pr_str(true), &val);
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

                        return Ok(lambda!(Lambda::new(
                            eval,
                            l[2].clone(),
                            args,
                            Rc::clone(env),
                        )));
                    }
                    MalVal::Symbol(s) if s == "eval" => {
                        let args = l[1].clone();
                        ast = eval(Ok(args), env)?;
                        repl_env = get_env_repl(env);
                        env = &repl_env;
                        continue;
                    }
                    MalVal::Symbol(s) if s == "quote" => {
                        return Ok(l[1].clone());
                    }
                    MalVal::Symbol(s) if s == "quasiquote" => {
                        ast = builtins::quasiquote(&l[1])?;
                        continue;
                    }
                    MalVal::Symbol(s) if s == "try*" => {
                        if l.len() < 3 {
                            ast = l[1].clone();
                            continue;
                        }
                        let try_ast = l[1].clone();
                        let catch_ast = l[2].clone();
                        let result = eval(Ok(try_ast), env);
                        let exc = match result {
                            Ok(v) => return Ok(v),
                            Err(MalError::Exception(e)) => e,
                            Err(MalError::Error(e)) => MalVal::Str(e),
                            _ => {
                                return Err(MalError::Error(
                                    "try* expects a try* and catch* block".to_string(),
                                ));
                            }
                        };

                        match catch_ast {
                            MalVal::List(seq, _) => {
                                if seq.len() != 3 {
                                    return Err(MalError::Error(
                                        "catch* expects a list of three arguments".to_string(),
                                    ));
                                }
                                match &seq[0] {
                                    MalVal::Symbol(s) if s == "catch*" => {
                                        let b = if let MalVal::Symbol(name) = seq[1].clone() {
                                            name
                                        } else {
                                            return Err(MalError::Error(
                                                "catch* expects a symbol as the first argument"
                                                    .to_string(),
                                            ));
                                        };
                                        let c = seq[2].clone();

                                        new_env = new_env!(Some(Rc::clone(env)));
                                        new_env.borrow_mut().set(&b, &exc)?;
                                        ast = c;
                                        env = &new_env;
                                        continue;
                                    }
                                    _ => return Ok(seq[0].clone()),
                                }
                            }
                            _ => {
                                return Err(MalError::Error("catch* expects a list".to_string()));
                            }
                        }
                    }
                    _ => match eval(Ok(l[0].clone()), env) {
                        Ok(func @ MalVal::Function(_, _)) => {
                            let argv: Vec<MalVal> = l
                                .iter()
                                .skip(1)
                                .map(|x| eval(Ok(x.clone()), env))
                                .collect::<Result<Vec<_>, _>>()?;

                            return func.apply(&argv);
                        }
                        Ok(MalVal::Lambda(f, _)) => {
                            if f.is_macro() {
                                ast = f.apply(&l[1..])?;
                            } else {
                                let argv: Vec<MalVal> = l
                                    .iter()
                                    .skip(1)
                                    .map(|x| eval(Ok(x.clone()), env))
                                    .collect::<Result<Vec<_>, _>>()?;

                                new_env = Rc::new(RefCell::new(f.bind(&argv)));
                                env = &new_env;
                                ast = (*f.ast).clone();
                            }
                            continue;
                        }
                        Ok(elt) => {
                            return Err(MalError::Error(format!(
                                "try to call a non-function: {}",
                                elt.pr_str(true)
                            )));
                        }
                        Err(e) => return Err(e),
                    },
                }
            }
            MalVal::Hashmap(h, _) => {
                let mut new_hm: HashMap<String, MalVal> = HashMap::new();
                let entries: Vec<_> = h.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                for (k, v) in entries {
                    new_hm.entry(k).insert_entry(eval(Ok(v), env).unwrap());
                }

                return Ok(hashmap!(new_hm));
            }
            _ => return Ok(ast),
        }
    } // TCO loop
}
