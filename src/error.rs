use std::result::Result as StdResult;
use id::Id;

#[derive(Debug)]
pub enum QDFError {
    SpaceDoesNotExists(Id),
    IncorrectSubdivisionsNumber(usize),
}

pub type Result<T> = StdResult<T, QDFError>;
