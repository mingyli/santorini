use santorini_common::{
    error::SantoriniError,
    input::{Column, Input, Position, Row},
    phase::Phase,
    player::{Player, Worker},
    state::State,
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

fn read_input() -> Input {
    Input {
        worker_location: read_position("Enter worker's current location"),
        worker_destination: read_position("Enter worker's destination location"),
        build_destination: read_position("Enter build location"),
    }
}

fn display(state: &State) {
    use colored::Colorize;
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
    let mut phase = Phase::InProgress(state);
    while let Phase::InProgress(state) = phase {
        display(&state);
        let input = read_input();
        println!("Applying input {:?}", input);
        phase = State::transition(state, &input);
    }
    println!("Game over!");
    println!("{:?}", phase);
}
