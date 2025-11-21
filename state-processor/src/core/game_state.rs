use crate::core::Terrain;
use crate::core::EntityMgmt;
use crate::core::TimeMgmt;
use pyo3::prelude::*;
use rand::distr::{Distribution, Uniform};
use std::f64::consts::PI;

#[derive(IntoPyObject)]
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
                Some(vector) => todo!(),
                None => continue //shouldn't happen, but this means the entity is non-existent
            }
        }

    }

    pub fn advance_state(&mut self) {
        self.time_mgmt.update();
        self.move_all_entities();
        self.entity_mgmt.update(&self.terrain_map);
    }
}
