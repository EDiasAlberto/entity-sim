//Defines core simulation handling logic
use noise::{Perlin, NoiseFn};
use rand::Rng;
use std::fmt;

const BASE_NOISE_SCALE: f64 = 6.0;

#[derive(Debug)]
enum Material {
    Mud,
    Grass,
    Ice,
}

struct MapPoint {
    height: u8,
    material: Material
}

impl fmt::Debug for MapPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}, with height {}", self.material, self.height)
    }
}

struct Terrain {
    width: u16,
    height: u16,
    depth: u8,
    map: Vec<MapPoint>
}

impl fmt::Debug for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Terrain: {}x{}x{}.", self.width, self.height, self.depth);
        for point in &self.map {
            let _ = write!(f, "({:#?},{}), ", point.material, point.height);
        }
        Ok(())
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Terrain of size {}x{}, and depth {}. Has {} Points", self.width, self.height, self.depth, self.map.len())
    }
}

impl Terrain {
    fn get_point(&self, x: u16, y: u16) -> &MapPoint {
        let idx: usize = ((y * self.width) + (x)).into();
        return &(self.map[idx]);
    }

    fn noise_to_height(&self, noise: f64) -> u8 {
        let normalised_noise = (noise + 1.0) / 2.0; // noise: [-1.0, 1.0] -> [0.0, 1.0] 
        let scaled_height = normalised_noise * (self.depth as f64); 
        (scaled_height as u8)
    }

    fn initialise_terrain(&mut self, noise: &Perlin) -> bool {
        let scale: f64 = BASE_NOISE_SCALE / (self.width as f64 * 0.5);
        println!("{scale}");
        for y in 0..self.height {
            for x in 0..self.width { 
                let noise_val = noise.get([x as f64 * scale, y as f64 * scale]);
                //println!("Point at {x}, {y} has noise {:.4}", &noise_val);
                let height_val = Self::noise_to_height(self, noise_val);
                self.map.push(MapPoint {height: height_val, material: Material::Grass})
            }
        }
        true
    }
}

pub fn generate_terrain(dimensions: (u16, u16, u8), seed: Option<u32>) {
    //TODO
    println!("Generating terrain!");
    /*
     * Idea: Generate map from optional provided seed
     * Get noise seed from args 
     * Generate noise map for altitude/verticality
     * Generate secondary noise map for terrain 
     * Use both to generate overall map data 
    */
    let mut rng = rand::rng();
    let random_seed: u32 = rng.random();
    let perlin = Perlin::new(random_seed);
    let (width, height, depth) = dimensions;
    let mut new_terrain = Terrain {width , height, depth, map: vec![]};
    new_terrain.initialise_terrain(&perlin);
    dbg!(new_terrain);


}

pub fn process_state() {
    //TODO
    println!("Processing passed state");
}
