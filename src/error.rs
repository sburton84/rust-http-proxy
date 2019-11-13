use std::fmt;

#[derive(Debug, Clone)]
pub struct NoHostError;

impl std::error::Error for NoHostError {}

impl fmt::Display for NoHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No host in request")
    }
}
