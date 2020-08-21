use super::player::Player;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
