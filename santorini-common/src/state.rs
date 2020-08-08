use std::fmt;

use crate::{
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
}

impl State {
    // TODO: Replace with builder API for initial workers.
    pub fn initial() -> State {
        State::default()
    }

    pub fn mut_space(&mut self, position: &Position) -> &mut Space {
        &mut self.board[position.row_index()][position.column_index()]
    }

    pub fn transition(mut self, input: &Input) -> Phase {
        self.apply_input(input);
        Phase::InProgress(self)
    }

    fn apply_input(&mut self, input: &Input) {
        let worker = self
            .mut_space(&input.worker_location)
            .mut_worker()
            .take()
            .unwrap();
        self.mut_space(&input.worker_destination)
            .mut_worker()
            .replace(worker)
            .unwrap_none();
        self.mut_space(&input.build_destination).tower.build();
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use colored::Colorize;
        writeln!(f, "  a b c d e")?;
        for (row_number, row) in self.board.iter().enumerate() {
            write!(f, "{} ", row_number)?;
            for space in row {
                write!(
                    f,
                    "{} ",
                    match space.worker() {
                        Some(worker) => match worker.player() {
                            Player::Red => format!("{}", space.tower).red(),
                            Player::Blue => format!("{}", space.tower).blue(),
                        },
                        None => format!("{}", space.tower).as_str().into(),
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
