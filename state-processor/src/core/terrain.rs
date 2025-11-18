use noise::{Perlin, NoiseFn};
use pyo3::prelude::*;
use std::fmt;


const BASE_NOISE_SCALE: f64 = 6.0;
const BASE_BIOME_SCALE: f64 = 0.6;

#[pyclass]
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Mud = 0,
    Grass = 1,
    Ice = 2,
}

#[derive(IntoPyObject)]
pub struct MapPoint {
    height: u8,
    material: u8, 
}

impl fmt::Debug for MapPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}, with height {}", self.material, self.height)
    }
}

#[derive(IntoPyObject)]
pub struct Terrain {
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
    pub fn new(width: u16, height: u16, depth: u8) -> Terrain {
        Terrain {width , height, depth, map: vec![]}
    }

    fn get_point(&self, x: u16, y: u16) -> &MapPoint {
        let idx: usize = ((y * self.width) + (x)).into();
        return &(self.map[idx]);
    }

    fn noise_range_change(&self, noise: f64, upper: f64) -> u8 {
        let normalised_noise = (noise + 1.0) / 2.0; // noise: [-1.0, 1.0] -> [0.0, 1.0] 
        let scaled_height = normalised_noise * upper; 
        scaled_height as u8
    }

    pub fn initialise_terrain(&mut self, noise: &Perlin) -> bool {
        let scale: f64 = BASE_NOISE_SCALE / (self.width as f64 * 0.5);
        let biome_scale = BASE_BIOME_SCALE / (self.width as f64 * 0.5);
        println!("{scale}");
        for y in 0..self.height {
            for x in 0..self.width { 
                let noise_val = noise.get([x as f64 * scale, y as f64 * scale]);
                let biome_noise = noise.get([x as f64 * biome_scale, y as f64 * biome_scale]);
                let height_val = Self::noise_range_change(self, noise_val, (self.depth as f64));
                let material_val = Self::noise_range_change(self, biome_noise, 4.0);
                self.map.push(MapPoint {height: height_val, material: material_val});
            }
        }
        true
    }
}

