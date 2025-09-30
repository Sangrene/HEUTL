use sqlx::Error as SQLXError;
use serde_json::Error as SerializeError;
use zen_engine::EvaluationError as ZenEngineError;

#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
    JsonError(String),
    NotFoundError(String),
    RuleEngineError(String),
}

impl From<SQLXError> for Error {
    fn from(error: SQLXError) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl From<SerializeError> for Error {
    fn from(error: SerializeError) -> Self {
        Error::JsonError(error.to_string())
    }
}

impl From<ZenEngineError> for Error {
    fn from(error: ZenEngineError) -> Self {
        Error::RuleEngineError(error.to_string())
    }
}