use crate::core::Terrain;
use glam::i32::IVec2;
use rand::distr::{Bernoulli, Distribution, Uniform};
use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::cmp::max;

mod death_calculations;

use death_calculations::{DeathCalc, WeibullDeath};

const MT_MIN_ENTITIES: usize = 375;

const BASE_MUD_SCALAR: f64 = 0.6;
const PROFICIENT_MUD_SCALAR: f64 = 0.8;

const BASE_ICE_SCALAR: f64 = 0.4;
const PROFICIENT_ICE_SCALAR: f64 = 0.7;

const PEAK_FERTILITY_AGE: f32 = 30.0;
const MIN_FERTILE_AGE: f32 = 15.0;
const MAX_FERTILE_AGE: f32 = 45.0;
const DEFAULT_ENTITY_EXPECTANCY: u8 = 70;
const DEFAULT_LIFE_STD_DEV: u8 = 15;
const DEFAULT_TIME_STEPS: u8 = 1;

fn calculate_material_speeds(is_climber: bool, is_skater: bool, grass_speed: f64) -> (f64, f64) {
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
    (mud_speed, ice_speed)
}

#[derive(IntoPyObject,Debug)]
pub struct Entity {
    age: u8,
    size: u8,
    death_age: u8,
    hunger: u8,
    is_alive: bool,
    is_male: bool,
    is_pregnant: bool,
    grass_speed: u8,
    mud_speed: u8,
    ice_speed: u8,
    location: (u16, u16),
    fertility: f32,
}

impl Entity {
    fn new(base_speed: u8, is_climber: bool, is_skater: bool, location: (u16, u16), is_male: bool, death_distr: &impl DeathCalc) -> Entity {
        let grass_speed: f64 = base_speed.into();

        let (mud_speed, ice_speed) = calculate_material_speeds(is_climber, is_skater, grass_speed);


        let death_age = death_distr.get_death_age();
        //println!("DYING AT: {}", death_age);

        Entity {age: 1, size: 1, hunger: 0, is_alive: true, is_pregnant: false, fertility: 0.0, grass_speed: (grass_speed as u8), mud_speed: (mud_speed as u8), ice_speed: (ice_speed as u8), location, is_male, death_age}
    }


    fn update_location(&mut self, new_loc: (u16, u16)) {
        self.location = new_loc;
    }

    fn grow_older(&mut self, age_increase: u8) {
        self.age = self.age + age_increase;
    }

    fn grow_bigger(&mut self, size_increase: u8) {
        self.size = self.size + size_increase;
    }

    // for now, fertility follows quadratic growth and decay about the peak age
    fn get_fertility_at_age(age: u8) -> f32 {
        let quadratic_scalar = (-100.0)/((PEAK_FERTILITY_AGE - MIN_FERTILE_AGE) * (PEAK_FERTILITY_AGE - MAX_FERTILE_AGE));
        (age as f32 - MIN_FERTILE_AGE) * (age as f32 - MAX_FERTILE_AGE) * quadratic_scalar

    }

    fn get_speed_for_material(&self, material: u8) -> u8 {
        match material {
            0 => self.mud_speed,
            1 => self.grass_speed,
            2 => self.ice_speed,
            _ => self.grass_speed,
        }
    }

    // temporary function for the time being, needs to be set to some 
    // reasonable distribution instead
    fn update_speed(&mut self) {
        if self.age <= 30 {
            self.grass_speed = self.grass_speed + 1;
        } else {
            self.grass_speed = max(self.grass_speed - 1, 0);
        }

    }

    fn update_fertility(&mut self) {
        if (self.age as f32) < MIN_FERTILE_AGE {
            self.fertility = 0.0;
        } else if (self.age as f32) > MAX_FERTILE_AGE {
            self.fertility = 0.0;
        } else {
            self.fertility = Self::get_fertility_at_age(self.age);
        }
    }

    fn do_death_check(&mut self) -> bool {
        self.is_alive = self.age <= self.death_age;
        !self.is_alive
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

    pub fn reset(&mut self) {
        *self = Self::new(self.spawn_area, self.area_dims);
        self.generate_random_entities(15, None, None);
    }

    pub fn get_num_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn generate_random_entities(&mut self, count: u16, life_exp: Option<u8>, life_std_dev: Option<u8>) {
        let between_x = Uniform::try_from(self.spawn_area.0..self.spawn_area.2).unwrap();
        let between_y = Uniform::try_from(self.spawn_area.1..self.spawn_area.3).unwrap();
        let gender = Bernoulli::new(0.5).unwrap();
        let expectancy = life_exp.unwrap_or(DEFAULT_ENTITY_EXPECTANCY);
        let deviation = life_std_dev.unwrap_or(DEFAULT_LIFE_STD_DEV);
        let death_distr = WeibullDeath::new(expectancy, deviation);
        let mut rng = rand::rng();
        for id in 0..count {
            let spawn_loc_x = between_x.sample(&mut rng);
            let spawn_loc_y = between_y.sample(&mut rng);
            let is_male = gender.sample(&mut rng);
            self.entities.insert(id as u16, Entity::new(30,false, false, (spawn_loc_x, spawn_loc_y), is_male, &death_distr));
        }
    }

    pub fn get_all_entity_locs(&self) -> HashMap<u16, (u16, u16)> {
        let mut map = HashMap::new();
        for (id, entity) in &self.entities {
            map.insert(*id, entity.location);
        }
        map
    }

    pub fn is_entity_alive(&self, id: u16) -> bool {
        self.entities.get(&id).unwrap().is_alive
    }

    fn clamp_entity_movement(map_dims: (u16, u16), curr_pos: (u16, u16), movement: IVec2) -> (u16, u16) {
        let curr_vec = IVec2::new(curr_pos.0.into(), curr_pos.1.into());
        let new_pos = curr_vec + movement;
        let clamped_pos_x = new_pos.x.clamp(0, map_dims.0 as i32 - 1);
        let clamped_pos_y = new_pos.y.clamp(0, map_dims.1 as i32 - 1);
        (clamped_pos_x.try_into().unwrap(), clamped_pos_y.try_into().unwrap())
    }


    pub fn get_entity_size(&self, id: u16) -> i8 {
        let ent = self.entities.get(&id);

        match ent {
            Some(x) => x.size as i8,
            None => -1,
        }
    }

    pub fn get_and_move_entity(&mut self, id: u16, movement: IVec2) -> bool { 
        let relevant_entity: &mut Entity = self.entities.get_mut(&id).unwrap();
        let new_location = {
            let loc = relevant_entity.location;
            Self::clamp_entity_movement(self.area_dims, loc, movement)
        };
        relevant_entity.update_location(new_location);

        true
    }
    
    /*
    fn calculate_pair_magnitude(&self, x: i32, y: i32) -> i32{
        ((x.pow(2) + y.pow(2)) as f64).sqrt() as i32
    }
    */

    fn generate_vector(speed: u8, direction: f64) -> IVec2 {
        let (rot_x, rot_y) = Self::calculate_rotated_components(speed as f64, direction);
        IVec2::new(rot_x, rot_y)
    }

    fn calculate_rotated_components(magnitude: f64, angle: f64) -> (i32, i32){
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let rotated_x = cos_angle*magnitude;
        let rotated_y = sin_angle*magnitude;
        (rotated_x as i32, rotated_y as i32)
    }

    fn calculate_new_entity_pos(area_dims: (u16, u16), map: &Terrain, entity: &Entity, direction: f64) -> (u16, u16) {
        //let entity = self.entities.get(entity_id).unwrap();
        let (x, y) = entity.location;
        let material = map.get_material(x, y);
        let speed = entity.get_speed_for_material(material);
        let movement_vector = Self::generate_vector(speed, direction);
        let new_location = Self::clamp_entity_movement(area_dims, (x, y), movement_vector);
        new_location
    }

    pub fn mt_random_move_all_entities(&mut self, map: &Terrain) {
        let between = Uniform::try_from(0.0..(2.0*PI)).unwrap();
        
        let _ = self.entities.par_iter_mut().for_each(|(_id, entity)| {
            if entity.is_alive{
                let mut rng = rand::rng();
                let direction = between.sample(&mut rng);
                let new_location = Self::calculate_new_entity_pos(self.area_dims, map, entity, direction);
                entity.update_location(new_location);
            }
        });

        /*
        for (id, entity) in &mut self.entities {
            if entity.is_alive {
                let direction = between.sample(&mut rng);
                let new_location = Self::calculate_new_entity_pos(self.area_dims, map, entity, direction);
                entity.update_location(new_location);
            }
        }
        */
    }

    pub fn random_move_all_entities(&mut self, map: &Terrain) {
        let between = Uniform::try_from(0.0..(2.0*PI)).unwrap();
        let mut rng = rand::rng();
        for (_id, entity) in &mut self.entities {
            if entity.is_alive {
                let direction = between.sample(&mut rng);
                let new_location = Self::calculate_new_entity_pos(self.area_dims, map, entity, direction);
                entity.update_location(new_location);
            }
        }
    }

    fn random_move_random_entities(&mut self, _map: &Terrain) {
        println!("Moving some entities randomly!");
    }
 
    // iterate over all entities, age up one year, attempt death
    fn age_all_entities(&mut self) {
        for (_id, entity) in &mut self.entities {
            if entity.is_alive {
                entity.grow_older(1);
                let died = entity.do_death_check();
                if !died {
                    entity.grow_bigger(1);
                    entity.update_speed();
                    entity.update_fertility();
                }
            }
        }
    }

    // use to update the state of stored entities (e.g. on event
    // occurring)
    pub fn advance_time(&mut self, map: &Terrain, steps: Option<u8>) {
        let num_steps = steps.unwrap_or(DEFAULT_TIME_STEPS);
        for _ in 0..num_steps {
            if self.get_num_entities() >= MT_MIN_ENTITIES {
                self.mt_random_move_all_entities(map);
            } else {
                self.random_move_all_entities(map);
            }
            self.age_all_entities();
        }
        
    }
}
