use super::{worker::Worker, tower::Tower};

#[derive(Default, Debug)]
pub struct Space {
    tower: Tower,
    worker: Option<Worker>,
}

impl Space {
    pub fn tower(&self) -> &Tower {
        &self.tower
    }

    pub fn mut_tower(&mut self) -> &mut Tower {
        &mut self.tower
    }

    pub fn worker(&self) -> &Option<Worker> {
        &self.worker
    }

    pub fn mut_worker(&mut self) -> &mut Option<Worker> {
        &mut self.worker
    }
}