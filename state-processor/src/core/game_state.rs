use crate::core::Terrain;
use crate::core::EntityMgmt;


pub struct GameState {
    terrain_map: Terrain,
    entity_mgmt: EntityMgmt,
}

impl GameState {
    pub fn new() -> GameState {
        let terrain = crate::core::generate_terrain((1000, 1000, 10), None); 
        let entities = EntityMgmt::new(200, 200, 400, 400);
        GameState {terrain_map: terrain, entity_mgmt: entities}
    }
}
