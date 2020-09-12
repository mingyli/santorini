use std::{convert::TryFrom, fmt};

use crate::error::SantoriniError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position(pub Column, pub Row);

impl Position {
    pub fn new(row: Row, column: Column) -> Position {
        Position(column, row)
    }

    pub fn row(&self) -> Row {
        self.1
    }

    pub fn row_index(&self) -> usize {
        self.row() as usize
    }

    pub fn column(&self) -> Column {
        self.0
    }

    pub fn column_index(&self) -> usize {
        self.column() as usize
    }

    pub fn adjacent_to(&self, other: &Position) -> bool {
        if self == other {
            false
        } else {
            let column_delta = (self.column_index() as i32 - other.column_index() as i32).abs();
            let row_delta = (self.row_index() as i32 - other.row_index() as i32).abs();
            column_delta <= 1 && row_delta <= 1
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.column(), self.row())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Column {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Column::A => 'A',
                Column::B => 'B',
                Column::C => 'C',
                Column::D => 'D',
                Column::E => 'E',
            }
        )
    }
}

impl TryFrom<char> for Column {
    type Error = SantoriniError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Column::A),
            'B' => Ok(Column::B),
            'C' => Ok(Column::C),
            'D' => Ok(Column::D),
            'E' => Ok(Column::E),
            _ => Err(SantoriniError::InvalidArgument(value.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Row::Zero => '0',
                Row::One => '1',
                Row::Two => '2',
                Row::Three => '3',
                Row::Four => '4',
            }
        )
    }
}

impl TryFrom<char> for Row {
    type Error = SantoriniError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            '0' => Ok(Row::Zero),
            '1' => Ok(Row::One),
            '2' => Ok(Row::Two),
            '3' => Ok(Row::Three),
            '4' => Ok(Row::Four),
            _ => Err(SantoriniError::InvalidArgument(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_computes_adjacency_correctly() {
        let adj = [
            Position::new(Row::Zero, Column::A),
            Position::new(Row::One, Column::A),
            Position::new(Row::Zero, Column::B),
            Position::new(Row::One, Column::B),
        ];
        let far = Position::new(Row::Four, Column::E);

        for i in 0..adj.len() {
            for j in 0..adj.len() {
                assert!((i == j) ^ &adj[i].adjacent_to(&adj[j]));
                assert!((i == j) ^ &adj[j].adjacent_to(&adj[i]));
            }
        }

        for pos in &adj {
            assert!(!pos.adjacent_to(&far));
            assert!(!&far.adjacent_to(pos));
        }
    }
}
