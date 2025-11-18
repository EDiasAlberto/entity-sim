use crate::core::Terrain;
use crate::core::EntityMgmt;
use crate::core::TimeMgmt;
use pyo3::prelude::*;

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

    pub fn advance_state(&mut self) {
        self.time_mgmt.update();
        self.entity_mgmt.update();
    }
}
