use std::error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeBaseError;

impl fmt::Display for TimeBaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid time base")
    }
}

impl error::Error for TimeBaseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
