use std::{convert::TryFrom, fmt};

use crate::error::SantoriniError;

#[derive(Copy, Clone, Debug)]
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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.column(), self.row())
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
