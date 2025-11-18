use rand::Rng;

const BASE_MUD_SCALAR: f64 = 0.6;
const PROFICIENT_MUD_SCALAR: f64 = 0.8;

const BASE_ICE_SCALAR: f64 = 0.4;
const PROFICIENT_ICE_SCALAR: f64 = 0.7;

pub struct Entity {
    age: u8,
    num_legs: u8,
    num_arms: u8,
    grass_speed: u8,
    mud_speed: u8,
    ice_speed: u8,
    location: (u16, u16),
}

impl Entity {
    fn new(num_legs: u8, num_arms: u8, base_speed: u8, is_climber: bool, is_skater: bool, location: (u16, u16)) -> Entity {
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

        Entity {age: 1, num_legs, num_arms, grass_speed: (grass_speed as u8), mud_speed: (mud_speed as u8), ice_speed: (ice_speed as u8), location}
    }
}

pub struct EntityMgmt {
    spawn_area: (u16, u16, u16, u16),
    entities: Vec<Entity>,
}

impl EntityMgmt {

    fn new(spawn_x_tl: u16, spawn_y_tl: u16, spawn_x_br: u16, spawn_y_br: u16) -> EntityMgmt{
        EntityMgmt {spawn_area: (spawn_x_tl, spawn_y_tl, spawn_x_br, spawn_y_br), entities: vec![]}
    }

    fn generate_random_entities(&mut self, count: u8) {
        let mut rng = rand::rng();
        let spawn_zone_width = self.spawn_area.2 - self.spawn_area.0;
        let spawn_zone_height = self.spawn_area.3 - self.spawn_area.1;
        for _ in 0..count {
            let random_x: u16 = (rng.random_range(0.0..1.0) * (spawn_zone_width as f64)) as u16;
            let random_y: u16 = (rng.random_range(0.0..1.0) * (spawn_zone_height as f64)) as u16;
            let spawn_loc_x = random_x + self.spawn_area.0;
            let spawn_loc_y = random_y + self.spawn_area.1;
            self.entities.push(Entity::new(2, 2, 10, false, false, (spawn_loc_x as u16, spawn_loc_y as u16)));
        }
    }
}
