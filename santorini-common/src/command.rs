use crate::{
    error::SantoriniError,
    objects::{state::State, tower::*},
    position::*,
};

pub trait Command {
    fn can_execute(&self, state: &State) -> Result<(), SantoriniError>;
    fn execute(&self, state: &mut State) -> Result<(), SantoriniError>;
    fn undo(&self, state: &mut State) -> Result<(), SantoriniError>;
}

pub struct MovementCommand {
    pub from: Position,
    pub to: Position,
}

impl Command for MovementCommand {
    fn can_execute(&self, state: &State) -> Result<(), SantoriniError> {
        // A move can execute if

        // 1) the `from` space has a worker
        let worker = match state.space(&self.from).worker() {
            Some(worker) => worker,
            None => {
                return Err(SantoriniError::InvalidArgument(format!(
                    "There is no worker at {}",
                    self.from
                )));
            }
        };

        // 2) the worker belongs to the current player's turn
        if worker.player() != state.current_player() {
            return Err(SantoriniError::InvalidArgument(format!(
                "Current player {} cannot control {} worker.",
                state.current_player(),
                worker.player()
            )));
        }

        // 3) it is a move phase
        // TODO

        // 4) it is a valid move in one of the 8 directions
        // TODO

        // 5) the `to` space is not occupied by another worker
        if state.space(&self.to).worker().is_some() {
            return Err(SantoriniError::InvalidMovement {
                from: self.from,
                to: self.to,
                details: "There is a worker blocking the move".into(),
            });
        }

        // 6) `to` space is not Domed
        if state.space(&self.to).tower().dome().is_some() {
            return Err(SantoriniError::InvalidMovement {
                from: self.from,
                to: self.to,
                details: "There is a dome blocking the move".into(),
            });
        }

        // 7) level of the `to` space <= level of the `from` space + 1
        let to_level = state.space(&self.to).tower().level();
        let from_level = state.space(&self.from).tower().level();
        if to_level > from_level.up() {
            return Err(SantoriniError::InvalidMovement {
                from: self.from,
                to: self.to,
                details: format!(
                    "Cannot move from level {} to level {}",
                    from_level, to_level
                ),
            });
        }

        Ok(())
    }

    fn execute(&self, state: &mut State) -> Result<(), SantoriniError> {
        self.can_execute(state)?;

        // Safely unwrap since we check can_execute?
        let worker = state.mut_space(&self.from).mut_worker().take().unwrap();
        state.mut_space(&self.to).mut_worker().replace(worker);

        Ok(())
    }

    fn undo(&self, _state: &mut State) -> Result<(), SantoriniError> {
        todo!()
    }
}

// Need to support specifying dome for characters like Ares.
pub struct BuildCommand {
    pub position: Position,
}

impl Command for BuildCommand {
    fn can_execute(&self, _state: &State) -> Result<(), SantoriniError> {
        panic!("Unimplemented")
    }

    fn execute(&self, state: &mut State) -> Result<(), SantoriniError> {
        let tower = state.mut_space(&self.position).mut_tower();
        if tower.dome().is_some() {
            Err(SantoriniError::InvalidBuild(self.position))
        } else {
            match tower.level() {
                Level::Three => {
                    tower.mut_dome().replace(Dome);
                }
                Level::Two => {
                    *tower.mut_level() = Level::Three;
                }
                Level::One => *tower.mut_level() = Level::Two,
                Level::Ground => *tower.mut_level() = Level::One,
            };
            // TODO: Consider separate command to terminate turn and swap players.
            *state.mut_current_player() = state.current_player().other();
            Ok(())
        }
    }

    fn undo(&self, _state: &mut State) -> Result<(), SantoriniError> {
        todo!()
    }
}
