use itertools::Itertools;

use santorini_common::{
    command::{Command, MovementCommand},
    objects::state::{State, StateBuilder},
    position::{Column, Position, Row},
};

#[test]
// TODO This integration test should be invalidated once we
// mandate that a worker must Move + Build
fn it_applies_move_commands() {
    let positions = vec![
        Position::new(Row::One, Column::A),
        Position::new(Row::Two, Column::A),
        Position::new(Row::Two, Column::B),
        Position::new(Row::Three, Column::C),
        Position::new(Row::Two, Column::B),
    ];

    let move_commands: Vec<MovementCommand> = positions
        .iter()
        .tuple_windows()
        .map(|(from, to)| MovementCommand {
            from: from.clone(),
            to: to.clone(),
        })
        .collect();
    let mut move_commands = move_commands.iter().map(|cmd| cmd as &dyn Command);

    let state = StateBuilder::new().add_red_worker(positions[0]).build();
    match move_commands.try_fold(state, State::apply_command) {
        Ok(state) => {
            let final_position = positions[positions.len() - 1];
            assert!(matches!(state.space(&final_position).worker(), Some(_)));
        }
        Err(e) => {
            assert!(false, "Unexpected error: {}", e);
        }
    };
}
