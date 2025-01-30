use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::vec::Vec;

pub mod environment;
pub mod error;
pub mod lambda;

// Assuming MalEnv is defined in the same module or another module
use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::lambda::Lambda;

pub type MalReturn = Result<MalVal, MalError>;
pub type MalFunctionArgs = Vec<MalVal>;
pub type MalFunction = fn(args: MalReturn, Rc<RefCell<MalEnv>>) -> MalReturn;

#[derive(Debug, Eq, Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Symbol(String),
    List(Vec<MalVal>),
    Vector(Vec<MalVal>),
    Hashmap(HashMap<String, MalVal>),
    Function(fn(MalFunctionArgs) -> MalReturn),
    Lambda(Rc<RefCell<Lambda>>),
}

fn escape_str(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

impl MalVal {
    pub fn apply(&self, args: Vec<MalVal>) -> MalReturn {
        match &self {
            MalVal::Function(f) => f(args),
            MalVal::Lambda(l) => l.borrow().apply(args),
            _ => Ok(self.clone()),
        }
    }

    pub fn pr_str(&self, print_readably: bool) -> String {
        let mut result = String::new();
        match self {
            MalVal::Nil => result.push_str("nil"),
            MalVal::Bool(v) => result.push_str(&v.to_string()),
            MalVal::Int(v) => result.push_str(&v.to_string()),
            MalVal::Symbol(v) => result.push_str(v),
            MalVal::Str(s) => {
                if print_readably {
                    if s.starts_with("\u{29e}") {
                        result.push(':');
                        result.push_str(escape_str(s).strip_prefix('\u{29e}').unwrap())
                    } else {
                        result.push_str(&format!("\"{}\"", escape_str(s)))
                    }
                } else if s.starts_with("\u{29e}") {
                    result.push(':');
                    result.push_str(s.strip_prefix('\u{29e}').unwrap())
                } else {
                    result.push_str(s)
                }
            }
            MalVal::List(l) => {
                result.push('(');
                let mut i = 0;

                while i < l.len() {
                    result.push_str(&l[i].pr_str(print_readably));
                    if i < l.len() - 1 {
                        result.push(' ');
                    }
                    i += 1;
                }
                result.push(')')
            }
            MalVal::Vector(v) => {
                result.push('[');
                let mut i = 0;

                while i < v.len() {
                    result.push_str(&v[i].pr_str(print_readably));
                    if i < v.len() - 1 {
                        result.push(' ');
                    }
                    i += 1;
                }
                result.push(']');
            }
            MalVal::Hashmap(h) => {
                result.push('{');

                for (i, (k, v)) in h.iter().enumerate() {
                    let key = k.clone();

                    if key.starts_with("\u{29e}") {
                        result.push_str(&format!(
                            ":{} {}",
                            key.strip_prefix('\u{29e}').unwrap(),
                            v.pr_str(print_readably)
                        ));
                    } else {
                        result.push_str(&format!("\"{}\" {}", key, v.pr_str(print_readably)));
                    }
                    if i < h.len() - 1 {
                        result.push(' ');
                    }
                }
                result.push('}');
            }
            MalVal::Function(_) => result.push_str("#<builtin>"),
            MalVal::Lambda(_) => result.push_str("#<function>"),
        }

        result
    }
}

impl PartialOrd for MalVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (MalVal::Int(a), MalVal::Int(b)) => a.partial_cmp(b),
            (MalVal::Str(a), MalVal::Str(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl PartialEq for MalVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MalVal::Nil, MalVal::Nil) => true,
            (MalVal::Bool(a), MalVal::Bool(b)) => a == b,
            (MalVal::Int(a), MalVal::Int(b)) => a == b,
            (MalVal::Str(a), MalVal::Str(b)) => a == b,
            (MalVal::Symbol(a), MalVal::Symbol(b)) => a == b,
            (MalVal::List(a), MalVal::List(b)) => a == b,
            (MalVal::Vector(a), MalVal::Vector(b)) => a == b,
            (MalVal::Vector(a), MalVal::List(b)) => a == b,
            (MalVal::List(a), MalVal::Vector(b)) => a == b,
            (MalVal::Hashmap(a), MalVal::Hashmap(b)) => a == b,
            _ => false,
        }
    }
}

impl Hash for MalVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MalVal::Str(s) => s.hash(state),
            MalVal::Symbol(s) => s.hash(state),
            _ => self.clone().pr_str(true).hash(state),
        }
    }
}
