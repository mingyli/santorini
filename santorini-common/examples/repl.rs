use santorini_common::{
    command::{BuildCommand, Command, MovementCommand},
    error::SantoriniError,
    position::{Column, Position, Row},
    phase::Phase, objects::{worker::Worker, state::State, player::Player},
};

fn read_position(message: &str) -> Position {
    use std::convert::TryFrom;
    use std::io;

    let foo = || -> Result<Position, SantoriniError> {
        println!("{}: ", message);
        let mut entry = String::new();
        io::stdin().read_line(&mut entry)?;
        match *entry.chars().collect::<Vec<char>>().as_slice() {
            [column, row, ..] => Ok(Position {
                column: Column::try_from(column)?,
                row: Row::try_from(row)?,
            }),
            _ => Err(SantoriniError::InvalidArgument(entry)),
        }
    };

    let mut result = foo();
    while let Err(err) = result {
        println!("{}. Let's try that again", err);
        result = foo();
    }
    result.unwrap()
}

fn get_move() -> Box<dyn Command> {
    Box::new(MovementCommand {
        from: read_position("worker from"),
        to: read_position("worker to"),
    })
}

fn get_build() -> Box<dyn Command> {
    Box::new(BuildCommand {
        position: read_position("build"),
    })
}

fn display(state: &State) {
    use colored::Colorize;
    println!("Current player {}", state.current_player());
    println!("  a b c d e");
    for (row_number, row) in state.board().iter().enumerate() {
        print!("{} ", row_number);
        for space in row {
            print!(
                "{} ",
                match space.worker() {
                    Some(worker) => match worker.player() {
                        Player::Red => format!("{}", space.tower()).red(),
                        Player::Blue => format!("{}", space.tower()).blue(),
                    },
                    None => format!("{}", space.tower()).as_str().into(),
                }
            );
        }
        println!();
    }
}

fn main() {
    let mut state = State::initial();
    *state
        .mut_space(&Position::new(Row::Zero, Column::A))
        .mut_worker() = Some(Worker::new(Player::Blue));
    *state
        .mut_space(&Position::new(Row::One, Column::A))
        .mut_worker() = Some(Worker::new(Player::Blue));
    *state
        .mut_space(&Position::new(Row::One, Column::B))
        .mut_worker() = Some(Worker::new(Player::Red));
    *state
        .mut_space(&Position::new(Row::Two, Column::B))
        .mut_worker() = Some(Worker::new(Player::Red));
    let mut factories = [get_move, get_build].iter().cycle();
    let mut phase = Phase::InProgress(state);
    while let Phase::InProgress(state) = phase {
        display(&state);
        let f = factories.next().unwrap();
        phase = State::transition(state, f().as_ref());
    }
    println!("Game over!");
    println!("{:?}", phase);
}
