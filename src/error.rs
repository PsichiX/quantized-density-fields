use id::ID;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum QDFError {
    SpaceDoesNotExists(ID),
    LevelDoesNotExists(ID),
    FieldDoesNotExists(ID),
}

pub type Result<T> = StdResult<T, QDFError>;
