//Defines core simulation handling logic
use noise::Perlin;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

mod terrain;
mod entity_management;
mod time_management;
mod game_state;

pub use terrain::Terrain;
pub use game_state::GameState;
pub use entity_management::EntityMgmt;
pub use time_management::TimeMgmt;

pub fn generate_terrain(dimensions: (u16, u16, u8), seed: Option<u32>) -> Terrain {
    println!("Generating terrain!");
    
    let random_seed: u32 = match seed {
        Some(x) => x,
        None => {let mut rng = rand::rng(); rng.random()},
    };

    //let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let perlin = Perlin::new(random_seed);
    let biome_perlin = Perlin::new(random_seed.wrapping_add(1000)); // Different seed for biomes
    let (width, height, depth) = dimensions;
    let mut new_terrain = Terrain::new(width, height, depth);
    //let definition_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    new_terrain.initialise_terrain(&perlin, &biome_perlin);
    //dbg!(new_terrain);
    //let init_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    //let def_time = definition_end - start;
    //let init_time = init_end - definition_end;
    //println!("{:#?} for definition, and {:#?} for init", def_time, init_time);
    new_terrain

}

pub fn generate_entities(spawn_zone: (u16, u16, u16, u16), terrain_dims: (u16, u16), amount: Option<u16>) -> EntityMgmt {
    println!("Generating entities!");
    let num_entities = amount.unwrap_or(15);

    let mut mgmt = EntityMgmt::new(spawn_zone, terrain_dims);
    mgmt.generate_random_entities(num_entities, None, None);
    mgmt
}

pub fn generate_clock(initial_time: Option<u32>) -> TimeMgmt {
    let start_time = match initial_time {
        Some(x) => x,
        None => 0,
    };
    TimeMgmt::new(start_time)
}

pub fn generate_game_state(map_size: (u16, u16, u8), spawn_zone: (u16, u16, u16, u16), starting_entities: Option<u16>) -> GameState {
    let time = generate_clock(None);
    let terrain = generate_terrain(map_size, None);
    let entities = generate_entities(spawn_zone, terrain.get_dims(), starting_entities);
    let gs = GameState::new(time, terrain, entities);
    gs
}

/*
pub fn process_state(mut gs: GameState, steps: Option<u8>) -> GameState{
    println!("Processing passed state");
    for i in 0..steps.unwrap_or(1) {
        gs.advance_state();
    }
    gs
}
*/
