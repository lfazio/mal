use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;
use crate::evaluation;
use crate::reader;

mod core;
mod math;
mod string;

pub fn register(env: Rc<RefCell<MalEnv>>) {
    core::register(env.clone());
    string::register(env.clone());
    math::register(env.clone());

    let _ = evaluation::eval(
        reader::read_str(
            "(def! not (fn* (a) (if a false true)))"), 
            Rc::clone(&env));

}
