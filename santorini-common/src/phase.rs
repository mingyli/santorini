use crate::{player::Player, state::State};

#[derive(Debug)]
pub enum Phase {
    InProgress(State),
    Win(State, Player),
}
