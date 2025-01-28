use crate::MalError;
use crate::MalVal;

pub fn print(output: Result<MalVal, MalError>) -> bool {
    match output {
        Ok(ast) => println!("{}", pr_str(&ast)),
        Err(MalError::Break(s)) => {
            println!("{}\nBye!", s);

            return false;
        }
        Err(e) => println!("{}", e),
    }

    true
}

pub fn pr_str(ast: &MalVal) -> String {
    format!("{}", ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MalError;
    use crate::MalVal;

    #[test]
    fn test_print_ok() {
        let output = Ok(MalVal::Int(42));
        assert!(print(output));
    }

    #[test]
    fn test_print_error() {
        let output = Err(MalError::Error("something went wrong".to_string()));
        assert!(print(output));
    }

    #[test]
    fn test_print_break() {
        let output = Err(MalError::Break("CTRL-C".to_string()));
        assert!(!print(output));
    }
}
