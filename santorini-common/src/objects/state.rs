use crate::{
    command::Command,
    error::SantoriniError,
    position::Position,
    phase::Phase,
};
use super::{player::Player, space::Space};


#[derive(Default, Debug)]
pub struct State {
    board: [[Space; 5]; 5],
    current_player: Player,
}

impl State {
    // TODO: Replace with builder API for initial workers.
    pub fn initial() -> State {
        State::default()
    }

    pub fn board(&self) -> &[[Space; 5]; 5] {
        &self.board
    }

    pub fn mut_space(&mut self, position: &Position) -> &mut Space {
        &mut self.board[position.row_index()][position.column_index()]
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn mut_current_player(&mut self) -> &mut Player {
        &mut self.current_player
    }

    pub fn transition(mut self, command: &dyn Command) -> Phase {
        if let Err(err) = self.apply_command(command) {
            eprintln!("{}", err);
        }
        Phase::InProgress(self)
    }

    // pub fn try_transition(mut self, input: &Input) -> Result<Phase, SantoriniError> {
    //     self.apply_input(input)?;
    //     Ok(Phase::InProgress(self))
    // }

    /// Validates that the input is sound. Performs any possible validation without
    /// modifying or cloning the game state.
    /// TODO: Consider moving to Command pattern with rollbacks and commits.
    // fn validate_input(&self, _input: &Input) -> Result<(), SantoriniError> {
    //     Ok(())
    // }

    /// Mutates the game state. Any errors surfaced here are not irrecoverable.
    fn apply_command(&mut self, command: &dyn Command) -> Result<(), SantoriniError> {
        command.execute(self)?;
        Ok(())
    }
}
