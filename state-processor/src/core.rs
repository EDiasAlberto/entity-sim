//Defines core simulation handling logic
use noise::Perlin;
use rand::Rng;

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

    let perlin = Perlin::new(random_seed);
    let biome_perlin = Perlin::new(random_seed.wrapping_add(1000)); // Different seed for biomes
    let (width, height, depth) = dimensions;
    let mut new_terrain = Terrain::new(width, height, depth);
    new_terrain.initialise_terrain(&perlin, &biome_perlin);
    //dbg!(new_terrain);
    new_terrain

}

pub fn generate_entities(spawn_zone: (u16, u16, u16, u16), amount: Option<u8>) -> EntityMgmt {
    println!("Generating entities!");
    let (x1, y1, x2, y2) = spawn_zone;
    let num_entities = match amount {
        Some(x) => x,
        None => 15,
    };

    let mut mgmt = EntityMgmt::new(x1, y1, x2, y2);
    mgmt.generate_random_entities(num_entities);
    mgmt
}

pub fn generate_clock(initial_time: Option<u32>) -> TimeMgmt {
    let start_time = match initial_time {
        Some(x) => x,
        None => 0,
    };
    TimeMgmt::new(start_time)
}

pub fn generate_game_state(map_size: (u16, u16, u8), spawn_zone: (u16, u16, u16, u16)) -> GameState {
    let time = generate_clock(None);
    let terrain = generate_terrain(map_size, None);
    let entities = generate_entities(spawn_zone, None);
    let mut gs = GameState::new(time, terrain, entities);
    gs.entity_mgmt.generate_random_entities(10);
    gs
}

pub fn process_state() {
    //TODO
    println!("Processing passed state");
}
