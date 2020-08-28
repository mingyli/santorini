use serde::{Deserialize, Serialize};

use super::{player::Player, space::Space, worker::Worker};
use crate::{command::Command, error::SantoriniError, phase::Phase, position::Position};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct State {
    board: [[Space; 5]; 5],
    current_player: Player,
}

impl State {
    pub fn builder() -> StateBuilder {
        StateBuilder::new()
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

    // I don't think this method should exist?
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
    pub fn apply_command(&mut self, command: &dyn Command) -> Result<(), SantoriniError> {
        command.execute(self)?;
        Ok(())
    }

    pub fn winner(&self) -> Option<Player> {
        return None;
    }
}

#[derive(Default)]
pub struct StateBuilder {
    red_workers: Vec<Position>,
    blue_workers: Vec<Position>,
}

impl StateBuilder {
    pub fn new() -> StateBuilder {
        StateBuilder::default()
    }

    pub fn add_red_worker(mut self, position: Position) -> StateBuilder {
        self.red_workers.push(position);
        self
    }

    pub fn add_blue_worker(mut self, position: Position) -> StateBuilder {
        self.blue_workers.push(position);
        self
    }

    pub fn build(self) -> State {
        let mut state = State::default();
        for position in self.red_workers {
            state
                .mut_space(&position)
                .mut_worker()
                .replace(Worker::new(Player::Red));
        }
        for position in self.blue_workers {
            state
                .mut_space(&position)
                .mut_worker()
                .replace(Worker::new(Player::Blue));
        }
        state
    }
}
