use crate::player::Player;
use crate::state::State;

#[derive(Debug)]
pub enum Phase {
    InProgress(State),
    Win(State, Player),
}
