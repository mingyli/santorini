pub mod command;
pub mod error;
pub mod objects;
pub mod phase;
pub mod position;

#[cfg(test)]
mod tests {
    use crate::{
        command::{BuildCommand, MovementCommand},
        objects::{
            player::Player,
            state::{State, StateBuilder},
            tower::Level,
        },
        phase::Phase,
        position::{Column, Position, Row},
    };

    #[test]
    fn it_works() {
        let state = StateBuilder::new()
            .add_blue_worker(Position::new(Row::One, Column::B))
            .add_blue_worker(Position::new(Row::Two, Column::B))
            .add_red_worker(Position::new(Row::One, Column::C))
            .add_red_worker(Position::new(Row::Two, Column::C))
            .build();

        assert_eq!(state.current_player(), Player::Red);
        let mut phase = State::transition(
            state,
            &MovementCommand {
                from: Position::new(Row::Two, Column::C),
                to: Position::new(Row::Three, Column::C),
            },
        );
        match phase {
            Phase::InProgress(state) => {
                assert_eq!(state.current_player(), Player::Red);
                assert!(state
                    .space(&Position::new(Row::Two, Column::C))
                    .worker()
                    .is_none(),);
                assert_eq!(
                    state
                        .space(&Position::new(Row::Three, Column::C))
                        .worker()
                        .as_ref()
                        .unwrap()
                        .player(),
                    Player::Red
                );
                phase = State::transition(
                    state,
                    &BuildCommand {
                        position: Position::new(Row::Two, Column::C),
                    },
                );
            }
            _ => panic!(),
        }
        match phase {
            Phase::InProgress(state) => {
                assert_eq!(state.current_player(), Player::Blue);
                assert_eq!(
                    state
                        .space(&Position::new(Row::Two, Column::C))
                        .tower()
                        .level(),
                    Level::One
                );
            }
            _ => panic!(),
        }
    }
}
