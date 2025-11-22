use crate::core::Terrain;
use glam::f32::Vec2;
use glam::u32::UVec2;
use glam::i32::IVec2;
use rand::Rng;
use rand::distr::{Bernoulli, Distribution, Uniform};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

const BASE_MUD_SCALAR: f64 = 0.6;
const PROFICIENT_MUD_SCALAR: f64 = 0.8;

const BASE_ICE_SCALAR: f64 = 0.4;
const PROFICIENT_ICE_SCALAR: f64 = 0.7;

#[derive(IntoPyObject,Debug)]
pub struct Entity {
    age: u8,
    hunger: u8,
    is_male: bool,
    is_pregnant: bool,
    grass_speed: u8,
    mud_speed: u8,
    ice_speed: u8,
    location: (u16, u16),
}

impl Entity {
    fn new(base_speed: u8, is_climber: bool, is_skater: bool, location: (u16, u16), is_male: bool) -> Entity {
        let grass_speed: f64 = base_speed.into();
        let mud_speed: f64;
        let ice_speed: f64;
        if is_climber {
            mud_speed = grass_speed * PROFICIENT_MUD_SCALAR;
        } else {
            mud_speed = grass_speed * BASE_MUD_SCALAR;
        }

        if is_skater {
            ice_speed = grass_speed * PROFICIENT_ICE_SCALAR;
        } else {
            ice_speed = grass_speed * BASE_ICE_SCALAR;
        }

        Entity {age: 1, hunger: 0, is_pregnant: false, grass_speed: (grass_speed as u8), mud_speed: (mud_speed as u8), ice_speed: (ice_speed as u8), location, is_male}
    }

    fn update_location(&mut self, new_loc: (u16, u16)) {
        self.location = new_loc;
    }
}

#[derive(IntoPyObject,Debug)]
pub struct EntityMgmt {
    spawn_area: (u16, u16, u16, u16),
    area_dims: (u16, u16),
    entities: HashMap<u16, Entity>,
}

impl EntityMgmt {

    pub fn new(spawn_area: (u16, u16, u16, u16), area_dims: (u16, u16)) -> EntityMgmt{
        EntityMgmt {spawn_area, area_dims, entities: HashMap::new()}
    }

    pub fn get_num_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn generate_random_entities(&mut self, count: u8) {
        let between_x = Uniform::try_from(self.spawn_area.0..self.spawn_area.2).unwrap();
        let between_y = Uniform::try_from(self.spawn_area.1..self.spawn_area.3).unwrap();
        let gender = Bernoulli::new(0.5).unwrap();
        let mut rng = rand::rng();
        for id in 0..count {
            let spawn_loc_x = between_x.sample(&mut rng);
            let spawn_loc_y = between_y.sample(&mut rng);
            let is_male = gender.sample(&mut rng);
            self.entities.insert((id as u16), Entity::new(30,false, false, (spawn_loc_x, spawn_loc_y), is_male));
        }
    }

    pub fn get_all_entity_locs(&self) -> HashMap<u16, (u16, u16)> {
        let mut map = HashMap::new();
        for (id, entity) in &self.entities {
            map.insert(*id, entity.location);
        }
        map
    }

    pub fn move_entity(&mut self, id: u16, movement: IVec2) -> bool {
        let relevant_entity: &mut Entity = self.entities.get_mut(&id).unwrap();
        let curr_pos = IVec2::new(relevant_entity.location.0.into(), relevant_entity.location.1.into());

        let new_pos = (curr_pos + movement);
        let clamped_pos_x = new_pos.x.clamp(0, self.area_dims.0 as i32);
        let clamped_pos_y = new_pos.y.clamp(0, self.area_dims.1 as i32);
        let new_location = (clamped_pos_x.try_into().unwrap(), clamped_pos_y.try_into().unwrap());
        relevant_entity.update_location(new_location);

        true
    }

    fn calculate_pair_magnitude(&self, x: i32, y: i32) -> i32{
        ((x.pow(2) + y.pow(2)) as f64).sqrt() as i32
    }

    fn calculate_rotated_components(&self, magnitude: f64, angle: f64) -> (i32, i32){
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let rotated_x = cos_angle*magnitude;
        let rotated_y = sin_angle*magnitude;
        (rotated_x as i32, rotated_y as i32)
    }

    pub fn generate_vector(&self, id: u16, material: u8, direction: f64) -> Option<IVec2>{
        if !(self.entities.contains_key(&id)) {
            return None;
        }
        let point = self.entities.get(&id).unwrap();
        
        let speed = match material {
            0 => point.mud_speed,
            1 => point.grass_speed,
            2 => point.ice_speed,
            _ => point.grass_speed,
        };
        let (x, y) = self.calculate_rotated_components(speed as f64, direction);
        Some(IVec2::new(x, y))

    }

    pub fn update(&mut self, map: &Terrain) {
        println!("Updated entities!");
    }
}
