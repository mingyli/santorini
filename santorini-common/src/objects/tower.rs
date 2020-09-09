use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dome;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tower {
    pub dome: Option<Dome>,
    pub level: Level,
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dome.is_some() {
            write!(f, "D")
        } else {
            self.level.fmt(f)
        }
    }
}

impl Tower {
    pub fn dome(&self) -> &Option<Dome> {
        &self.dome
    }

    pub fn mut_dome(&mut self) -> &mut Option<Dome> {
        &mut self.dome
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn mut_level(&mut self) -> &mut Level {
        &mut self.level
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

impl Level {
    pub fn up(self) -> Level {
        match self {
            Level::Ground => Level::One,
            Level::One => Level::Two,
            Level::Two | Level::Three => Level::Three,
        }
    }
}
