use std::convert::TryFrom;

#[derive(Debug)]
pub struct Input {
    pub worker_location: Position,
    pub worker_destination: Position,
    pub build_destination: Position,
}

#[derive(Debug)]
pub struct Position {
    pub column: Column,
    pub row: Row,
}

impl Position {
    pub fn new(row: Row, column: Column) -> Position {
        Position { row, column }
    }

    pub fn row(&self) -> Row {
        self.row
    }

    pub fn row_index(&self) -> usize {
        self.row as usize
    }

    pub fn column(&self) -> Column {
        self.column
    }

    pub fn column_index(&self) -> usize {
        self.column as usize
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

impl TryFrom<char> for Column {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Column::A),
            'B' => Ok(Column::B),
            'C' => Ok(Column::C),
            'D' => Ok(Column::D),
            'E' => Ok(Column::E),
            _ => Err("fkj"),
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

impl TryFrom<char> for Row {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Row::Zero),
            '1' => Ok(Row::One),
            '2' => Ok(Row::Two),
            '3' => Ok(Row::Three),
            '4' => Ok(Row::Four),
            _ => Err("lksj"),
        }
    }
}
