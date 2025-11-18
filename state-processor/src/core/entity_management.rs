use rand::Rng;
use rand::distr::{Bernoulli, Distribution, Uniform};
use std::collections::HashMap;

const BASE_MUD_SCALAR: f64 = 0.6;
const PROFICIENT_MUD_SCALAR: f64 = 0.8;

const BASE_ICE_SCALAR: f64 = 0.4;
const PROFICIENT_ICE_SCALAR: f64 = 0.7;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct EntityMgmt {
    spawn_area: (u16, u16, u16, u16),
    entities: HashMap<u16, Entity>,
}

impl EntityMgmt {

    pub fn new(spawn_x_tl: u16, spawn_y_tl: u16, spawn_x_br: u16, spawn_y_br: u16) -> EntityMgmt{
        EntityMgmt {spawn_area: (spawn_x_tl, spawn_y_tl, spawn_x_br, spawn_y_br), entities: HashMap::new()}
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
            self.entities.insert((id as u16), Entity::new(2,false, false, (spawn_loc_x, spawn_loc_y), is_male));
        }
    }

    pub fn get_all_entity_locs(&self) -> HashMap<u16, (u16, u16)> {
        let mut map = HashMap::new();
        for (id, entity) in &self.entities {
            map.insert(*id, entity.location);
        }
        map
        
    }
}
