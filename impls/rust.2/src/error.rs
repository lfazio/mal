use std::fmt;

#[derive(Debug, Clone)]
pub enum MalError {
    Break(String),
    Readline(String),
    Error(String),
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MalError::Break(s) => write!(f, "Break: {}", s),
            MalError::Readline(s) => write!(f, "Error: Readline: {}", s),
            MalError::Error(s) => write!(f, "Error: {}", s),
        }
    }
}
