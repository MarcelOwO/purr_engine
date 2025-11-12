use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, WindowError>;

#[derive(Clone, Debug)]
pub(crate) enum WindowError {
    CreationFailed,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in window logic")
    }
}
