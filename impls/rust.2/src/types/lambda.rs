use std::cell::RefCell;
use std::rc::Rc;

use super::MalFunction;
use super::MalReturn;
use super::MalVal;
use super::environment::MalEnv;

// macro to create la list
use crate::list;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda {
    eval: MalFunction,
    pub ast: Rc<MalVal>,
    pub args: Rc<Vec<MalVal>>,
    pub env: Rc<RefCell<MalEnv>>,
    pub is_macro: bool,
}

impl Lambda {
    pub fn new(eval: MalFunction, ast: MalVal, args: MalVal, env: Rc<RefCell<MalEnv>>) -> Lambda {
        Lambda {
            eval,
            ast: Rc::new(ast),
            args: Rc::new(match args {
                MalVal::List(a, _) | MalVal::Vector(a, _) => a.to_vec(),
                _ => vec![],
            }),
            env,
            is_macro: false,
        }
    }

    pub fn new_macro(
        eval: MalFunction,
        ast: MalVal,
        args: MalVal,
        env: Rc<RefCell<MalEnv>>,
    ) -> Lambda {
        Lambda {
            eval,
            ast: Rc::new(ast),
            args: Rc::new(match args {
                MalVal::List(a, _) | MalVal::Vector(a, _) => a.to_vec(),
                _ => vec![],
            }),
            env,
            is_macro: true,
        }
    }

    pub fn bind(&self, argv: &[MalVal]) -> MalEnv {
        let mut new_env = MalEnv::new(Some(self.env.clone()));
        let mut remaining = false;
        for (i, arg) in self.args.iter().enumerate() {
            match &arg {
                MalVal::Symbol(s) if s == "&" => {
                    remaining = true;
                }
                MalVal::Symbol(s) => {
                    if remaining {
                        set_env!(new_env, s, &list!(argv[i - 1..].to_vec()));
                    } else {
                        set_env!(new_env, s, &argv[i]);
                    }
                }
                _ => (),
            }
        }

        new_env
    }

    pub fn apply(&self, argv: &[MalVal]) -> MalReturn {
        let new_env = Rc::new(RefCell::new(self.bind(argv)));
        (self.eval)(Ok((*self.ast).clone()), &new_env)
    }

    pub fn is_macro(&self) -> bool {
        self.is_macro
    }

    pub fn get_args(&self) -> Vec<MalVal> {
        (*self.args).clone()
    }

    pub fn get_ast(&self) -> MalVal {
        (*self.ast).clone()
    }
}

impl PartialOrd for Lambda {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
