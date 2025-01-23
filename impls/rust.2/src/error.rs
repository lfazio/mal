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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malerror_display() {
        let break_error = MalError::Break("CTRL-C".to_string());
        assert_eq!(format!("{}", break_error), "Break: CTRL-C");

        let readline_error = MalError::Readline("unexpected error".to_string());
        assert_eq!(
            format!("{}", readline_error),
            "Error: Readline: unexpected error"
        );

        let generic_error = MalError::Error("something went wrong".to_string());
        assert_eq!(format!("{}", generic_error), "Error: something went wrong");
    }
}
