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
}

impl Entity {
    fn new(num_legs: u8, num_arms: u8, base_speed: u8, is_climber: bool, is_skater: bool) -> Entity {
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

        Entity {age: 1, num_legs, num_arms, grass_speed: (grass_speed as u8), mud_speed: (mud_speed as u8), ice_speed: (ice_speed as u8)}
    }
}

pub struct EntityMgmt {
    entities: Vec<Entity>
}
