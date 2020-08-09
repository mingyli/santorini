use santorini_common::{
    command::{BuildCommand, Command, MovementCommand},
    error::SantoriniError,
    objects::{
        player::Player,
        state::{State, StateBuilder},
        worker::Worker,
    },
    phase::Phase,
    position::{Column, Position, Row},
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
        from: read_position("Worker from"),
        to: read_position("Worker to"),
    })
}

fn get_build() -> Box<dyn Command> {
    Box::new(BuildCommand {
        position: read_position("Build"),
    })
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
    println!(
        "Current player {}",
        match state.current_player() {
            Player::Red => "Red".red(),
            Player::Blue => "Blue".blue(),
        }
    );
}

fn main() {
    let state = StateBuilder::new()
        .add_blue_worker(Position::new(Row::One, Column::B))
        .add_blue_worker(Position::new(Row::Two, Column::B))
        .add_red_worker(Position::new(Row::One, Column::C))
        .add_red_worker(Position::new(Row::Two, Column::C))
        .build();
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
