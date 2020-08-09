use thiserror::Error;

use crate::input::Position;

#[derive(Error, Debug)]
pub enum SantoriniError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Invalid build")]
    InvalidBuild,

    #[error("Invalid movement")]
    InvalidMovement { from: Position, to: Position },

    #[error(transparent)]
    Other(#[from] std::io::Error),
}
