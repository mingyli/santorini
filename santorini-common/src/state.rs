use std::fmt;

use crate::{
    error::SantoriniError,
    input::{Input, Position},
    phase::Phase,
    player::{Player, Worker},
    tower::Tower,
};

#[derive(Default, Debug)]
pub struct Space {
    tower: Tower,
    worker: Option<Worker>,
}

impl Space {
    pub fn tower(&self) -> &Tower {
        &self.tower
    }

    pub fn worker(&self) -> &Option<Worker> {
        &self.worker
    }

    pub fn mut_worker(&mut self) -> &mut Option<Worker> {
        &mut self.worker
    }
}

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

    pub fn transition(mut self, input: &Input) -> Phase {
        if let Err(err) = self.apply_input(input) {
            eprintln!("{}", err);
        }
        self.current_player = self.current_player.other();
        Phase::InProgress(self)
    }

    // pub fn try_transition(mut self, input: &Input) -> Result<Phase, SantoriniError> {
    //     self.apply_input(input)?;
    //     Ok(Phase::InProgress(self))
    // }

    /// Validates that the input is sound. Performs any possible validation without
    /// modifying or cloning the game state.
    /// TODO: Consider moving to Command pattern with rollbacks and commits.
    fn validate_input(&self, _input: &Input) -> Result<(), SantoriniError> {
        Ok(())
    }

    /// Mutates the game state. Any errors surfaced here are not irrecoverable.
    fn apply_input(&mut self, input: &Input) -> Result<(), SantoriniError> {
        let worker = self
            .mut_space(&input.worker_location)
            .mut_worker()
            .take()
            .ok_or_else(|| {
                SantoriniError::InvalidArgument(format!(
                    "There is no worker at {}",
                    input.worker_location
                ))
            })?;
        if worker.player() != self.current_player {
            return Err(SantoriniError::InvalidArgument(format!(
                "Current player {} cannot control {} worker.",
                self.current_player,
                worker.player()
            )));
        }
        let previous = self
            .mut_space(&input.worker_destination)
            .mut_worker()
            .replace(worker);
        if previous.is_some() {
            return Err(SantoriniError::InvalidMovement {
                from: input.worker_location.clone(),
                to: input.worker_destination.clone(),
            });
        }
        self.mut_space(&input.build_destination).tower.build()?;
        Ok(())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  a b c d e")?;
        for (row_number, row) in self.board.iter().enumerate() {
            write!(f, "{} ", row_number)?;
            for space in row {
                write!(f, "{}", space.tower)?;
                write!(
                    f,
                    "{} ",
                    match space.worker() {
                        Some(worker) => match worker.player() {
                            Player::Red => 'R',
                            Player::Blue => 'B',
                        },
                        None => ' ',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
