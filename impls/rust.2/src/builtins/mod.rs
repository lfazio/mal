use std::{cell::RefCell, rc::Rc};

use crate::types::environment::MalEnv;

mod core;
mod math;
mod string;

pub fn register(env: Rc<RefCell<MalEnv>>) {
    core::register(env.clone());
    string::register(env.clone());
    math::register(env.clone());
}
