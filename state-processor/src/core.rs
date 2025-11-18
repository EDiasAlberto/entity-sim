//Defines core simulation handling logic
use noise::Perlin;
use rand::Rng;

mod terrain;
mod entity_management;
mod game_state;

pub use terrain::Terrain;
pub use game_state::GameState;
pub use entity_management::EntityMgmt;

pub fn generate_terrain(dimensions: (u16, u16, u8), seed: Option<u32>) -> Terrain {
    println!("Generating terrain!");
    
    let random_seed: u32 = match seed {
        Some(x) => x,
        None => {let mut rng = rand::rng(); rng.random()},
    };

    let perlin = Perlin::new(random_seed);
    let (width, height, depth) = dimensions;
    let mut new_terrain = Terrain::new(width, height, depth);
    new_terrain.initialise_terrain(&perlin);
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

pub fn generate_game_state() -> GameState {
    let terrain = generate_terrain((1000, 1000, 10), None);
    let entities = generate_entities((200, 200, 400, 400), None);
    let mut gs = GameState::new(terrain, entities);
    gs.entity_mgmt.generate_random_entities(10);
    gs
}

pub fn process_state() {
    //TODO
    println!("Processing passed state");
}
