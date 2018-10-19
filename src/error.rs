use std::result::Result as StdResult;
use id::Id;

#[derive(Debug)]
pub enum QDFError {
    SpaceDoesNotExists(Id),
    IncorrectDimensionsNumber(usize),
    SpaceIsNotSubdivided(Id),
}

pub type Result<T> = StdResult<T, QDFError>;
