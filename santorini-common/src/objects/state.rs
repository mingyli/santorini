use serde::{Deserialize, Serialize};

use super::{
    player::Player,
    space::Space,
    tower::{Level, Tower},
    worker::Worker,
};
use crate::{
    command::Command,
    error::SantoriniError,
    phase::Phase,
    position::{Column, Position, Row},
};

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

    pub fn space(&self, position: &Position) -> &Space {
        &self.board[position.row_index()][position.column_index()]
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

    // Returns a winner, assuming that a legal state has exactly one winner
    pub fn winner(&self) -> Option<Player> {
        self.board.iter().flatten().find_map(|space| {
            if space.tower().level() == Level::Three {
                space.worker().as_ref().and_then(|w| Some(w.player()))
            } else {
                None
            }
        })
    }
}

#[derive(Default)]
pub struct StateBuilder {
    red_workers: Vec<Position>,
    blue_workers: Vec<Position>,
    towers: Vec<(Position, Tower)>,
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

    pub fn add_tower(mut self, position: Position, tower: Tower) -> StateBuilder {
        self.towers.push((position, tower));
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

        for (position, tower) in self.towers {
            *(state.mut_space(&position).mut_tower()) = tower;
        }
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_winner_when_won() {
        let pos = Position::new(Row::One, Column::A);
        let mut state = StateBuilder::new().add_red_worker(pos).build();
        *(state.mut_space(&pos).mut_tower().mut_level()) = Level::Three;

        assert_eq!(state.winner(), Some(Player::Red));
    }

    #[test]
    fn it_finds_no_winner_when_not_won() {
        let pos = Position::new(Row::One, Column::A);
        let state = StateBuilder::new().add_red_worker(pos).build();

        assert_eq!(state.winner(), None);
    }
}
