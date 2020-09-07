use itertools::Itertools;

use santorini_common::{
    command::{Command, MovementCommand},
    objects::state::StateBuilder,
    phase::*,
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
    let phase = Phase::InProgress(state);
    let result = move_commands.try_fold(phase, Phase::apply);

    assert!(matches!(result, Ok(_)));

    if let Ok(Phase::InProgress(final_state)) = result {
        let final_position = positions[positions.len() - 1];
        assert!(matches!(
            final_state.space(&final_position).worker(),
            Some(_)
        ));
    } else {
        assert!(false);
    }
}
