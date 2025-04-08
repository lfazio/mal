use std::cell::RefCell;
use std::rc::Rc;

use super::MalFunction;
use super::MalReturn;
use super::MalVal;
use super::environment::MalEnv;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda {
    eval: MalFunction,
    pub ast: Rc<MalVal>,
    args: Rc<Vec<MalVal>>,
    pub env: Rc<RefCell<MalEnv>>,
}

impl Lambda {
    pub fn new(eval: MalFunction, ast: MalVal, args: MalVal, env: Rc<RefCell<MalEnv>>) -> Lambda {
        Lambda {
            eval,
            ast: Rc::new(ast),
            args: Rc::new(match args {
                MalVal::List(a) | MalVal::Vector(a) => a.to_vec(),
                _ => vec![],
            }),
            env,
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
                        let _ =
                            set_env!(new_env, s, &MalVal::List(Rc::new(argv[i - 1..].to_vec())));
                    } else {
                        let _ = set_env!(new_env, s, &argv[i]);
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
}

impl PartialOrd for Lambda {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
