use thiserror::*;

#[derive(Error, Debug)]
pub enum UnitError {
    #[error("Error in query: {0}")]
    Query(String),

    #[error("Unit doesn´t exist : {0}")]
    BadUnit(String),

    #[error("Impossible to parse {0}")]
    ParseError(String),

    #[error("{0}")]
    Custom(String),
}
