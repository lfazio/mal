use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::MalError;
use crate::MalVal;

#[derive(Debug)]
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

    pub fn get(&self, symbol: &str) -> Result<MalVal, MalError> {
        match self.symbols.get(symbol) {
            Some(sval) => Ok(sval.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(symbol),
                None => Err(MalError::Error(format!("'{}' not found", symbol))),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MalVal;

    #[test]
    fn test_malenv_new() {
        let env = MalEnv::new(None);
        assert!(env.symbols.is_empty());
    }

    #[test]
    fn test_malenv_put_and_get() {
        let mut env = MalEnv::new(None);
        let symbol = "key".to_string();
        let value = MalVal::Int(42);

        env.set(&symbol, value.clone());
        assert_eq!(env.get(&symbol).unwrap(), value);
    }

    #[test]
    fn test_malenv_get_nonexistent() {
        let env = MalEnv::new(None);
        let symbol = "nonexistent".to_string();
        assert!(env.get(&symbol).is_err());
    }

    #[test]
    fn test_malenv_update_existing() {
        let mut env = MalEnv::new(None);
        let symbol = "key".to_string();
        let value1 = MalVal::Int(42);
        let value2 = MalVal::Int(43);

        env.set(&symbol, value1.clone());
        assert_eq!(env.get(&symbol).unwrap(), value1);

        env.set(&symbol, value2.clone());
        assert_eq!(env.get(&symbol).unwrap(), value2);
    }

    #[test]
    fn test_malenv_multiple_symbols() {
        let mut env = MalEnv::new(None);
        let symbol1 = "key1".to_string();
        let symbol2 = "key2".to_string();
        let value1 = MalVal::Int(42);
        let value2 = MalVal::Int(43);

        env.set(&symbol1, value1.clone());
        env.set(&symbol2, value2.clone());

        assert_eq!(env.get(&symbol1).unwrap(), value1);
        assert_eq!(env.get(&symbol2).unwrap(), value2);
    }

    #[test]
    fn test_malenv_nested_hashmaps() {
        let mut env = MalEnv::new(None);
        let symbol = "key".to_string();
        let nested_symbol = "nested_key".to_string();
        let nested_value = MalVal::Int(42);

        let mut nested_env = MalEnv::new(None);
        nested_env.set(&nested_symbol, nested_value.clone());

        env.set(&symbol, MalVal::Hashmap(nested_env.symbols.clone()));

        match env.get(&symbol).unwrap().clone() {
            MalVal::Hashmap(h) => {
                assert_eq!(*h.get(&nested_symbol).unwrap(), nested_value);
            }
            _ => panic!("Expected MalVal::Hashmap"),
        }
    }
}
