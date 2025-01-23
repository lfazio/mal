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

fn pr_str<'a>(ast: &'a MalVal) -> String {
    format!("{}", ast)
}
