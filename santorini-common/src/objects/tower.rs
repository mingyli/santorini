use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dome;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tower {
    dome: Option<Dome>,
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
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
