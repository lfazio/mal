use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::vec::Vec;

#[derive(Debug, Eq, Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Symbol(String),
    List(Vec<MalVal>),
    Vector(Vec<MalVal>),
    Hashmap(HashMap<MalVal, MalVal>),
    Function(fn(MalVal) -> MalVal),
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

impl fmt::Display for MalVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MalVal::Nil => write!(f, "nil"),
            MalVal::Bool(b) => write!(f, "{}", b),
            MalVal::Int(i) => write!(f, "{}", i),
            MalVal::Str(s) => {
                if s.starts_with("\u{29e}") {
                    write!(f, ":{}", escape_str(s).strip_prefix('\u{29e}').unwrap())
                } else {
                    write!(f, "\"{}\"", escape_str(s))
                }
            }
            MalVal::Symbol(s) => write!(f, "{}", s),
            MalVal::List(l) => {
                let _ = write!(f, "(");
                let mut i = 0;

                while i < l.len() {
                    _ = write!(f, "{}", l[i]);
                    if i < l.len() - 1 {
                        _ = write!(f, " ");
                    }
                    i += 1;
                }
                write!(f, ")")
            }
            MalVal::Vector(v) => {
                let _ = write!(f, "[");
                let mut i = 0;

                while i < v.len() {
                    _ = write!(f, "{}", v[i]);
                    if i < v.len() - 1 {
                        _ = write!(f, " ");
                    }
                    i += 1;
                }
                write!(f, "]")
            }
            MalVal::Hashmap(h) => {
                let _ = write!(f, "{{");

                for (i, (k, v)) in h.iter().enumerate() {
                    _ = write!(f, "{} {}", k, v);
                    if i < h.len() - 1 {
                        _ = write!(f, " ");
                    }
                }
                write!(f, "}}")
            }
            MalVal::Function(_func) => {
                write!(f, "<#builtin>")
            }
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
            (MalVal::Hashmap(a), MalVal::Hashmap(b)) => a == b,
            (MalVal::Function(a), MalVal::Function(b)) => std::ptr::fn_addr_eq(*a, *b),
            _ => format!("{:?}", self) == format!("{:?}", other),
        }
    }
}

impl Hash for MalVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MalVal::Str(s) => s.hash(state),
            MalVal::Symbol(s) => s.hash(state),
            _ => format!("{}", self).hash(state),
        }
    }
}
