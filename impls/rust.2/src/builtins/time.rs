use std::time::SystemTime;
use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::types::error::MalError;
use crate::types::{MalReturn, MalVal};

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    builtin_register!(env, "time-ms", time_ms);
}

fn time_ms(args: &[MalVal]) -> MalReturn {
    if args.len() > 0 {
        return Err(MalError::Error(
            "time-ms does not expect any argument".to_string(),
        ));
    }

    Ok(MalVal::Int(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    ))
}
