use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    builtin_register!(env, "throw", r#throw);
}

fn r#throw(args: &[MalVal]) -> MalReturn {
    if args.len() != 1 {
        return Err(MalError::Error(
            "throw expects only one argument".to_string(),
        ));
    }

    Err(MalError::Exception(args[0].clone()))
}
