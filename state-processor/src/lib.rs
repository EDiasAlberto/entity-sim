// Exposes function wrappers for creating python lib
mod core;

use pyo3::prelude::*;

#[pyfunction]
fn run_terrain_gen(width: u16, height: u16, depth: u8) -> core::Terrain{
    core::generate_terrain((width, height, depth), None)
}

#[pyfunction] 
fn generate_game_state() -> core::GameState {
    core::generate_game_state()
}

#[pymodule]
fn state_processor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_terrain_gen, m)?);
    m.add_function(wrap_pyfunction!(generate_game_state, m)?);
    Ok(())
}
