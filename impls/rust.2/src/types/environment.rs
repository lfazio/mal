use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MalEnv {
    symbols: HashMap<String, MalVal>,
    outer: Option<Rc<RefCell<MalEnv>>>,
}

impl MalEnv {
    pub fn new(outer: Option<Rc<RefCell<MalEnv>>>) -> MalEnv {
        MalEnv {
            symbols: HashMap::<String, MalVal>::new(),
            outer,
        }
    }

    pub fn set(&mut self, symbol: &str, o: MalVal) {
        self.symbols
            .entry(symbol.to_string())
            .and_modify(|val| *val = o.clone())
            .or_insert(o);
    }

    pub fn get(&self, symbol: &str) -> MalReturn {
        match self.symbols.get(symbol) {
            Some(sval) => Ok(sval.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(symbol),
                None => Err(MalError::Error(format!("'{}' not found", symbol))),
            },
        }
    }
}

