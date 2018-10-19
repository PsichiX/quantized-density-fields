use id::Id;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum QDFError {
    SpaceDoesNotExists(Id),
    IncorrectSubdivisionsNumber(usize),
}

pub type Result<T> = StdResult<T, QDFError>;
