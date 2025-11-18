use noise::{Perlin, NoiseFn};
use numpy::{PyArray2, PyArray1, PyArrayMethods};
use pyo3::prelude::*;
use std::fmt;


const BASE_NOISE_SCALE: f64 = 6.0;
const BASE_BIOME_SCALE: f64 = 0.8;  // Much larger scale = bigger, smoother biomes

#[pyclass]
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Mud = 0,
    Grass = 1,
    Ice = 2,
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct MapPoint {
    height: u8,
    material: u8, 
}

impl fmt::Debug for MapPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}, with height {}", self.material, self.height)
    }
}

#[pyclass]
pub struct Terrain {
    #[pyo3(get)]
    width: u16,
    #[pyo3(get)]
    height: u16,
    #[pyo3(get)]
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

#[pymethods]
impl Terrain {
    
    // Returns terrain materials and heights as numpy arrays
    // Returns a tuple of (materials, heights) where each is a 2D numpy array
    fn get_map_data<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray2<u8>>, Bound<'py, PyArray2<u8>>)> {
        let size = (self.height as usize, self.width as usize);
        
        // Create numpy arrays
        let materials = PyArray2::<u8>::zeros(py, size, false);
        let heights = PyArray2::<u8>::zeros(py, size, false);
        
        // Get mutable slices
        unsafe {
            let materials_slice = materials.as_slice_mut()?;
            let heights_slice = heights.as_slice_mut()?;
            
            for (i, point) in self.map.iter().enumerate() {
                materials_slice[i] = point.material;
                heights_slice[i] = point.height;
            }
        }
        
        Ok((materials, heights))
    }
}

impl Terrain {
    pub fn new(width: u16, height: u16, depth: u8) -> Terrain {
        Terrain {width , height, depth, map: vec![]}
    }

    pub fn get_point(&self, x: u16, y: u16) -> &MapPoint {
        let idx: usize = ((y as usize * self.width as usize) + (x as usize));
        return &(self.map[idx]);
    }

    fn noise_range_change(&self, noise: f64, upper: f64) -> u8 {
        let normalised_noise = (noise + 1.0) / 2.0; // noise: [-1.0, 1.0] -> [0.0, 1.0] 
        let scaled_height = normalised_noise * upper; 
        scaled_height as u8
    }

    fn biome_noise_to_material(&self, noise: f64) -> u8 {
        let normalised = (noise + 1.0) / 2.0; // noise: [-1.0, 1.0] -> [0.0, 1.0]
        
        // Adjust thresholds to reduce grass proportion
        // Grass now occupies the middle 25% instead of 33%
        if normalised < 0.375 {
            0  // Mud
        } else if normalised < 0.625 {
            1  // Grass (reduced range)
        } else {
            2  // Ice
        }
    }

    pub fn initialise_terrain(&mut self, noise: &Perlin, biome_noise: &Perlin) -> bool {
        let scale: f64 = BASE_NOISE_SCALE / (self.width as f64 * 0.5);
        let biome_scale = BASE_BIOME_SCALE / (self.width as f64 * 0.5);
        
        println!("{scale}");
        for y in 0..self.height {
            for x in 0..self.width { 
                let noise_val = noise.get([x as f64 * scale, y as f64 * scale]);
                
                // Multi-octave biome noise for more organic shapes
                let biome_x = x as f64 * biome_scale;
                let biome_y = y as f64 * biome_scale;
                
                // Layer multiple frequencies (octaves) for natural-looking biomes
                let biome_noise_1 = biome_noise.get([biome_x, biome_y]) * 1.0;
                let biome_noise_2 = biome_noise.get([biome_x * 2.5, biome_y * 2.5]) * 0.5;
                let biome_noise_3 = biome_noise.get([biome_x * 5.0, biome_y * 5.0]) * 0.25;
                
                let combined_biome_noise = (biome_noise_1 + biome_noise_2 + biome_noise_3) / 1.75;
                
                let height_val = Self::noise_range_change(self, noise_val, self.depth as f64);
                let material_val = Self::biome_noise_to_material(self, combined_biome_noise);
                self.map.push(MapPoint {height: height_val, material: material_val});
            }
        }
        true
    }
}

