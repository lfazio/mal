use crate::MalError;
use crate::MalVal;

pub fn print(output: Result<MalVal, MalError>) -> bool {
    match output {
        Ok(ast) => println!("{}", ast.pr_str(true)),
        Err(MalError::Break(s)) => {
            println!("{}\nBye!", s);

            return false;
        }
        Err(e) => println!("{}", e),
    }

    true
}
