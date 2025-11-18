use pyo3::prelude::*;

#[pyclass]
pub struct TimeMgmt {
    time: u32,
}

impl TimeMgmt {
    pub fn new(start_time: u32) -> TimeMgmt {
        TimeMgmt {time: start_time}
    }
}
