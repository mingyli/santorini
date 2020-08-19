use crate::{
    error::SantoriniError,
    objects::{state::State, tower::Level},
    position::Position,
};

pub trait Command {
    fn execute(&self, state: &mut State) -> Result<(), SantoriniError>;
    fn undo(&self, state: &mut State) -> Result<(), SantoriniError>;
}

pub struct MovementCommand {
    pub from: Position,
    pub to: Position,
}

impl Command for MovementCommand {
    fn execute(&self, state: &mut State) -> Result<(), SantoriniError> {
        let worker = state
            .mut_space(&self.from)
            .mut_worker()
            .take()
            .ok_or_else(|| {
                SantoriniError::InvalidArgument(format!("There is no worker at {}", self.from,))
            })?;
        if worker.player() != state.current_player() {
            return Err(SantoriniError::InvalidArgument(format!(
                "Current player {} cannot control {} worker.",
                state.current_player(),
                worker.player()
            )));
        }
        let previous = state.mut_space(&self.to).mut_worker().replace(worker);
        if previous.is_some() {
            return Err(SantoriniError::InvalidMovement {
                from: self.from,
                to: self.to,
            });
        }
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
    fn execute(&self, state: &mut State) -> Result<(), SantoriniError> {
        let tower = state.mut_space(&self.position).mut_tower();
        if tower.dome().is_some() {
            Err(SantoriniError::InvalidBuild(self.position))
        } else {
            match tower.level() {
                Level::Three => {
                    tower.mut_dome().replace(());
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
