use std::collections::HashMap;

use crate::MalError;
use crate::MalVal;

pub struct MalEnv {
    symbols: HashMap<String, MalVal>,
}

impl MalEnv {
    pub fn new() -> MalEnv {
        MalEnv {
            symbols: HashMap::<String, MalVal>::new(),
        }
    }

    pub fn put(&mut self, symbol: &str, o: MalVal) {
        self.symbols
            .entry(symbol.to_string())
            .and_modify(|val| *val = o.clone())
            .or_insert(o);
    }

    pub fn get(&mut self, symbol: &String) -> Result<MalVal, MalError> {
        match self.symbols.get(symbol) {
            Some(sval) => Ok(sval.clone()),
            None => Err(MalError::Error(format!("'{}' not found", symbol))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MalVal;

    #[test]
    fn test_malenv_new() {
        let env = MalEnv::new();
        assert!(env.symbols.is_empty());
    }

    #[test]
    fn test_malenv_put_and_get() {
        let mut env = MalEnv::new();
        let symbol = "key".to_string();
        let value = MalVal::Int(42);

        env.put(&symbol, value.clone());
        assert_eq!(env.get(&symbol).unwrap(), value);
    }

    #[test]
    fn test_malenv_get_nonexistent() {
        let mut env = MalEnv::new();
        let symbol = "nonexistent".to_string();
        assert!(env.get(&symbol).is_err());
    }

    #[test]
    fn test_malenv_update_existing() {
        let mut env = MalEnv::new();
        let symbol = "key".to_string();
        let value1 = MalVal::Int(42);
        let value2 = MalVal::Int(43);

        env.put(&symbol, value1.clone());
        assert_eq!(env.get(&symbol).unwrap(), value1);

        env.put(&symbol, value2.clone());
        assert_eq!(env.get(&symbol).unwrap(), value2);
    }

    #[test]
    fn test_malenv_multiple_symbols() {
        let mut env = MalEnv::new();
        let symbol1 = "key1".to_string();
        let symbol2 = "key2".to_string();
        let value1 = MalVal::Int(42);
        let value2 = MalVal::Int(43);

        env.put(&symbol1, value1.clone());
        env.put(&symbol2, value2.clone());

        assert_eq!(env.get(&symbol1).unwrap(), value1);
        assert_eq!(env.get(&symbol2).unwrap(), value2);
    }

    #[test]
    fn test_malenv_nested_hashmaps() {
        let mut env = MalEnv::new();
        let symbol = "key".to_string();
        let nested_symbol = "nested_key".to_string();
        let nested_value = MalVal::Int(42);

        let mut nested_env = MalEnv::new();
        nested_env.put(&nested_symbol, nested_value.clone());

        env.put(&symbol, MalVal::Hashmap(nested_env.symbols.clone()));

        match env.get(&symbol).unwrap().clone() {
            MalVal::Hashmap(h) => {
                assert_eq!(*h.get(&nested_symbol).unwrap(), nested_value);
            }
            _ => panic!("Expected MalVal::Hashmap"),
        }
    }
}
