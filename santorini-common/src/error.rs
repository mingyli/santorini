use thiserror::Error;

use crate::position::Position;

#[derive(Error, Debug)]
pub enum SantoriniError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Invalid build on position: {0}")]
    InvalidBuild(Position),

    #[error("Invalid movement from {from} to {to}: {details}")]
    InvalidMovement {
        from: Position,
        to: Position,
        details: String,
    },

    #[error(transparent)]
    Other(#[from] std::io::Error),
}
