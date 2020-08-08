use santorini_common::{
    input::{Column, Input, Position, Row},
    phase::Phase,
    player::{Player, Worker},
    state::State,
};

fn read_position(message: &str) -> Position {
    use std::convert::TryFrom;
    use std::io;

    println!("{}: ", message);
    let mut entry = String::new();
    io::stdin().read_line(&mut entry).unwrap();
    match *entry
        .to_uppercase()
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
    {
        [column, row, ..] => Position {
            column: Column::try_from(column).unwrap(),
            row: Row::try_from(row).unwrap(),
        },
        _ => panic!("TODO: handle retry"),
    }
}

fn read_input() -> Input {
    Input {
        worker_location: read_position("Enter worker's current location"),
        worker_destination: read_position("Enter worker's destination location"),
        build_destination: read_position("Enter build location"),
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
        println!("{}", state);
        let input = read_input();
        println!("Applying input {:?}", input);
        phase = State::transition(state, &input);
    }
    println!("Game over!");
    println!("{:?}", phase);
}
