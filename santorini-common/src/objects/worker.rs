use super::player::Player;

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