use rand::prelude::*;
use rand_distr::Weibull;
use statrs::function::gamma::gamma;

const WEIBULL_SHAPE_DEFAULT: f64 = 5.0;

pub trait DeathCalc {
    fn new(exp: u8) -> impl DeathCalc;
    fn get_death_age(&self) -> u8;
}

pub struct WeibullDeath {
    expected_age: u8,
    distribution: Weibull<f64>,
}

impl WeibullDeath {
    fn calculate_coefficients(exp: u8) -> (f64, f64) {
        let shape = WEIBULL_SHAPE_DEFAULT;
        let denominator = gamma(1.0 + (1.0/shape));
        let scale = (exp as f64)/ denominator;
        (shape, scale.into())
    }
}

impl DeathCalc for WeibullDeath {
    fn new(exp: u8) -> impl DeathCalc {
        let (shape, scale) = WeibullDeath::calculate_coefficients(exp);
        WeibullDeath {expected_age: exp, distribution: Weibull::new(shape, scale).unwrap()}
    }

    fn get_death_age(&self) -> u8 {
        let mut rng = rand::rng();
        self.distribution.sample(&mut rng) as u8
    }

}
