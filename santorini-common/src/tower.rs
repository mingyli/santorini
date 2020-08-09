use std::fmt;

use crate::error::SantoriniError;

#[derive(Default, Debug)]
pub struct Tower {
    dome: Option<()>,
    level: Level,
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dome.is_some() {
            write!(f, "D")
        } else {
            write!(f, "{}", self.level as usize)
        }
    }
}

impl Tower {
    pub fn build(&mut self) -> Result<(), SantoriniError> {
        if self.dome.is_some() {
            Err(SantoriniError::InvalidBuild)
        } else {
            match self.level {
                Level::Three => self.build_dome(),
                _ => self.build_block(),
            }
        }
    }

    pub fn build_dome(&mut self) -> Result<(), SantoriniError> {
        match self.dome {
            Some(_) => Err(SantoriniError::InvalidBuild),
            None => {
                self.dome = Some(());
                Ok(())
            }
        }
    }

    pub fn build_block(&mut self) -> Result<(), SantoriniError> {
        self.level.up()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Level {
    Ground = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl Default for Level {
    fn default() -> Level {
        Level::Ground
    }
}

impl Level {
    fn up(&mut self) -> Result<(), SantoriniError> {
        match self {
            Level::Ground => {
                *self = Level::One;
                Ok(())
            }
            Level::One => {
                *self = Level::Two;
                Ok(())
            }
            Level::Two => {
                *self = Level::Three;
                Ok(())
            }
            Level::Three => Err(SantoriniError::InvalidBuild),
        }
    }
}
