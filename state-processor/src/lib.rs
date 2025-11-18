// Exposes function wrappers for creating python lib
mod core;

use pyo3::prelude::*;

#[pyfunction]
fn validate_and_run_terrain_gen(width: u16, height: u16, depth: u8) -> core::Terrain{
    core::generate_terrain((width, height, depth), None)
}

#[pymodule]
fn state_processor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_and_run_terrain_gen, m)?)
}
