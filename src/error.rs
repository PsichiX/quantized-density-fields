use id::ID;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum QDFError {
    SpaceDoesNotExists(ID),
    IncorrectSubdivisionsNumber(usize),
}

pub type Result<T> = StdResult<T, QDFError>;
