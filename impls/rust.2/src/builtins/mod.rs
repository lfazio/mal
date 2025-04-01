use std::{cell::RefCell, rc::Rc};

use crate::evaluation;
use crate::reader;
use crate::types::environment::MalEnv;

mod core;
mod math;
mod string;

pub fn register(env: &Rc<RefCell<MalEnv>>) {
    core::register(env);
    string::register(env);
    math::register(env);

    let _ = evaluation::eval(
        reader::read_str("(def! not (fn* (a) (if a false true)))"),
        env,
    );
}
