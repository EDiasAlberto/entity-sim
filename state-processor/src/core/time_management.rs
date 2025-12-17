use pyo3::prelude::*;

#[pyclass]
pub struct TimeMgmt {
    time: u32,
}

impl TimeMgmt {
    pub fn new(start_time: u32) -> TimeMgmt {
        TimeMgmt {time: start_time}
    }

    pub fn update(&mut self) -> u32 {
        self.time += 1;
        self.time
    }

    pub fn reset(&mut self) {
        self.time = 0;
    }
}
