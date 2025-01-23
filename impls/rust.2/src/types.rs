use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug, Eq, Clone)]
pub enum MalVal {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Symbol(String),
    List(Rc<RefCell<Vec<MalVal>>>),
    Vector(Rc<RefCell<Vec<MalVal>>>),
    Hashmap(Rc<RefCell<HashMap<String, MalVal>>>),
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
            MalVal::Bool(v) => write!(f, "{}", v),
            MalVal::Int(v) => write!(f, "{}", v),
            MalVal::Symbol(v) => write!(f, "{}", v),
            MalVal::Str(s) => {
                if s.starts_with("\u{29e}") {
                    write!(f, ":{}", escape_str(s).strip_prefix('\u{29e}').unwrap())
                } else {
                    write!(f, "\"{}\"", escape_str(s))
                }
            }
            MalVal::List(l) => {
                let _ = write!(f, "(");
                let mut i = 0;

                while i < l.borrow().len() {
                    _ = write!(f, "{}", l.borrow()[i]);
                    if i < l.borrow().len() - 1 {
                        _ = write!(f, " ");
                    }
                    i += 1;
                }
                write!(f, ")")
            }
            MalVal::Vector(v) => {
                let _ = write!(f, "[");
                let mut i = 0;

                while i < v.borrow().len() {
                    _ = write!(f, "{}", v.borrow()[i]);
                    if i < v.borrow().len() - 1 {
                        _ = write!(f, " ");
                    }
                    i += 1;
                }
                write!(f, "]")
            }
            MalVal::Hashmap(h) => {
                let _ = write!(f, "{{");

                for (i, (k, v)) in h.borrow().iter().enumerate() {
                    _ = write!(f, "{} {}", MalVal::Str(k.clone()), v);
                    if i < h.borrow().len() - 1 {
                        _ = write!(f, " ");
                    }
                }
                write!(f, "}}")
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
            _ => false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_malval_display() {
        assert_eq!(format!("{}", MalVal::Nil), "nil");
        assert_eq!(format!("{}", MalVal::Bool(true)), "true");
        assert_eq!(format!("{}", MalVal::Int(42)), "42");
        assert_eq!(format!("{}", MalVal::Str("hello".to_string())), "\"hello\"");
        assert_eq!(format!("{}", MalVal::Symbol("sym".to_string())), "sym");

        let list = MalVal::List(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        assert_eq!(format!("{}", list), "(1 2)");

        let vector = MalVal::Vector(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        assert_eq!(format!("{}", vector), "[1 2]");

        let mut hashmap = HashMap::new();
        hashmap.insert("key".to_string(), MalVal::Int(42));
        let hashmap_val = MalVal::Hashmap(Rc::new(RefCell::new(hashmap)));
        assert_eq!(format!("{}", hashmap_val), "{\"key\" 42}");
    }

    #[test]
    fn test_malval_equality() {
        assert_eq!(MalVal::Nil, MalVal::Nil);
        assert_eq!(MalVal::Bool(true), MalVal::Bool(true));
        assert_eq!(MalVal::Int(42), MalVal::Int(42));
        assert_eq!(
            MalVal::Str("hello".to_string()),
            MalVal::Str("hello".to_string())
        );
        assert_eq!(
            MalVal::Symbol("sym".to_string()),
            MalVal::Symbol("sym".to_string())
        );

        let list1 = MalVal::List(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        let list2 = MalVal::List(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        assert_eq!(list1, list2);

        let vector1 = MalVal::Vector(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        let vector2 = MalVal::Vector(Rc::new(RefCell::new(vec![MalVal::Int(1), MalVal::Int(2)])));
        assert_eq!(vector1, vector2);

        let mut hashmap1 = HashMap::new();
        hashmap1.insert("key".to_string(), MalVal::Int(42));
        let hashmap_val1 = MalVal::Hashmap(Rc::new(RefCell::new(hashmap1)));

        let mut hashmap2 = HashMap::new();
        hashmap2.insert("key".to_string(), MalVal::Int(42));
        let hashmap_val2 = MalVal::Hashmap(Rc::new(RefCell::new(hashmap2)));

        assert_eq!(hashmap_val1, hashmap_val2);
    }
}
