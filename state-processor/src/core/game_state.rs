use crate::core::Terrain;
use crate::core::EntityMgmt;
use crate::core::TimeMgmt;
use numpy::{PyArray2, PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use rand::distr::{Distribution, Uniform};
use std::f64::consts::PI;

#[pyclass]
pub struct GameState {
    pub time_mgmt: TimeMgmt,
    pub terrain_map: Terrain,
    pub entity_mgmt: EntityMgmt,
}

impl GameState {
    pub fn new(time: TimeMgmt, terrain: Terrain, entities: EntityMgmt) -> GameState {
        GameState {time_mgmt: time, terrain_map: terrain, entity_mgmt: entities}
    }

    pub fn move_all_entities(&mut self) {
        let between = Uniform::try_from(0.0..(2.0*PI)).unwrap();
        let mut rng = rand::rng();
        for (id, location) in self.entity_mgmt.get_all_entity_locs() {
            let (x, y) = location;
            let material = self.terrain_map.get_material(x, y);
            let direction = between.sample(&mut rng);
            let movement_vector = self.entity_mgmt.generate_vector(id, material, direction);
            match movement_vector {
                Some(vector) => self.entity_mgmt.move_entity(id, vector),
                None => continue //shouldn't happen, but this means the entity is non-existent
            };
        }
    }
}

#[pymethods]
impl GameState {
    pub fn get_terrain_map(&self) -> (u16, u16) {
        self.terrain_map.get_dims()
    }


    fn get_entity_locations<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray2<u16>>)> {
        let num_entities = self.entity_mgmt.get_num_entities();
        let map = self.entity_mgmt.get_all_entity_locs();
        let mut rows: Vec<Vec<u16>> = Vec::with_capacity(map.len());

        for (id, (a, b)) in map {
            rows.push(vec![id as u16, a, b]);
        }
        
        Ok(PyArray2::from_vec2(py, &rows).unwrap().to_owned())
    }

    fn get_map_data<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray2<u8>>, Bound<'py, PyArray2<u8>>)> {
        let (width, height) = self.terrain_map.get_dims();
        let size = (height as usize, width as usize);
        
        // Create numpy arrays
        let materials = PyArray2::<u8>::zeros(py, size, false);
        let heights = PyArray2::<u8>::zeros(py, size, false);
        
        // Get mutable slices
        unsafe {
            let materials_slice = materials.as_slice_mut()?;
            let heights_slice = heights.as_slice_mut()?;
            
            for (i, point) in self.terrain_map.map.iter().enumerate() {
                materials_slice[i] = point.material;
                heights_slice[i] = point.height;
            }
        }
        
        Ok((materials, heights))
    }

    pub fn advance_state(&mut self) {
        self.time_mgmt.update();
        self.move_all_entities();
        self.entity_mgmt.update(&self.terrain_map);
    }
}
