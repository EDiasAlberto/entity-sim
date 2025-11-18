//Defines core simulation handling logic
use noise::{Perlin, NoiseFn};
use pyo3::IntoPyObject;
use pyo3::prelude::*;
use rand::Rng;
use std::fmt;

mod terrain;
mod map_point;

pub use terrain::Terrain;

pub fn generate_terrain(dimensions: (u16, u16, u8), seed: Option<u32>) -> terrain::Terrain {
    println!("Generating terrain!");
    
    let random_seed: u32 = match seed {
        Some(x) => x,
        None => {let mut rng = rand::rng(); rng.random()},
    };

    let perlin = Perlin::new(random_seed);
    let (width, height, depth) = dimensions;
    let mut new_terrain = terrain::Terrain::new(width, height, depth);
    new_terrain.initialise_terrain(&perlin);
    //dbg!(new_terrain);
    new_terrain

}

pub fn process_state() {
    //TODO
    println!("Processing passed state");
}
