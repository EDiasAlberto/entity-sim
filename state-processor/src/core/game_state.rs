use crate::core::Terrain;
use crate::core::EntityMgmt;


pub struct GameState {
    pub terrain_map: Terrain,
    pub entity_mgmt: EntityMgmt,
}

impl GameState {
    pub fn new(terrain: Terrain, entities: EntityMgmt) -> GameState {
        GameState {terrain_map: terrain, entity_mgmt: entities}
    }

}
