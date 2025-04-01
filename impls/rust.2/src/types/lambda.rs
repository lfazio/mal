use std::cell::RefCell;
use std::rc::Rc;

use super::environment::MalEnv;
use super::MalFunction;
use super::MalReturn;
use super::MalVal;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda {
    eval: MalFunction,
    pub ast: MalVal,
    args: Vec<MalVal>,
    pub env: Rc<RefCell<MalEnv>>,
}

impl Lambda {
    pub fn new(eval: MalFunction, ast: MalVal, args: MalVal, env: Rc<RefCell<MalEnv>>) -> Lambda {
        Lambda {
            eval,
            ast,
            env,
            args: match args {
                MalVal::List(a) | MalVal::Vector(a) => a.to_vec(),
                _ => vec![],
            },
        }
    }

    pub fn bind(&self, argv: Vec<MalVal>) -> MalEnv {
        let mut new_env = MalEnv::new(Some(self.env.clone()));
        let mut remaining = false;
        for (i, arg) in self.args.iter().enumerate() {
            match &arg {
                MalVal::Symbol(s) if s == "&" => {
                    remaining = true;
                }
                MalVal::Symbol(s) => {
                    if remaining {
                        new_env.set(s, &MalVal::List(Rc::new(argv[i - 1..].to_vec())));
                    } else {
                        new_env.set(s, &argv[i])
                    }
                }
                _ => (),
            }
        }

        new_env
    }

    pub fn apply(&self, argv: Vec<MalVal>) -> MalReturn {
        let new_env = Rc::new(RefCell::new(self.bind(argv)));
        (self.eval)(Ok(self.ast.clone()), &new_env)
    }
}

impl PartialOrd for Lambda {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
