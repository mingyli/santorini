use crate::objects::{player::Player, state::State};
use crate::{command::Command, error::SantoriniError};

#[derive(Debug)]
pub enum Phase {
    InProgress(State),
    Win(State, Player),
}

impl Phase {
    fn apply(self, command: &dyn Command) -> Result<Phase, SantoriniError> {
        match self {
            Phase::InProgress(mut state) => {
                &state.apply_command(command)?;
                Ok(match state.winner() {
                    Some(player) => Phase::Win(state, player),
                    None => Phase::InProgress(state),
                })
            }
            Phase::Win(_, _) => Ok(self),
        }
    }
}
