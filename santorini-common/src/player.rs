use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Worker {
    player: Player,
}

impl Worker {
    pub fn new(player: Player) -> Worker {
        Worker { player }
    }

    pub fn player(&self) -> Player {
        self.player
    }
}
