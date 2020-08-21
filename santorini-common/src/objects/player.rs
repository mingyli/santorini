use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

impl Default for Player {
    fn default() -> Player {
        Player::Red
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Red => write!(f, "Red"),
            Player::Blue => write!(f, "Blue"),
        }
    }
}
