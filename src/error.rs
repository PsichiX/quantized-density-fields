use id::ID;
use std::result::Result as StdResult;

/// Defines Quantized Density Fields errors.
#[derive(Debug)]
pub enum QDFError {
    /// Tells that specified space does not exists in container.
    SpaceDoesNotExists(ID),
    /// Tells that specified level does not exists in container.
    LevelDoesNotExists(ID),
    /// Tells that specified field does not exists in container.
    FieldDoesNotExists(ID),
}

/// Alias for standard result with `QDFError` error type.
pub type Result<T> = StdResult<T, QDFError>;
