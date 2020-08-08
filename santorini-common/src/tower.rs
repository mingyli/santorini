use std::fmt;

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
    pub fn build(&mut self) {
        if self.dome.is_some() {
            panic!("TODO: handle dome")
        } else {
            match self.level {
                Level::Three => self.build_dome(),
                _ => self.build_block(),
            }
        }
    }

    pub fn build_dome(&mut self) {
        self.dome = Some(())
    }

    pub fn build_block(&mut self) {
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
    fn up(&mut self) {
        match self {
            Level::Ground => *self = Level::One,
            Level::One => *self = Level::Two,
            Level::Two => *self = Level::Three,
            Level::Three => panic!("TODO: handle level three"),
        }
    }
}
