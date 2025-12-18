use rand::prelude::*;
use rand_distr::Weibull;
use roots::SimpleConvergency;
use roots::{find_root_brent, SearchError};
use statrs::function::gamma::gamma;

pub trait DeathCalc {
    fn new(exp: u8, std_dev: u8) -> impl DeathCalc;
    fn get_death_age(&self) -> u8;
}

pub struct WeibullDeath {
    expected_age: u8,
    distribution: Weibull<f64>,
}

impl WeibullDeath {

    // f(x) = 0, when x==mean of the weibull data
    fn f(k: f64, sigma2: f64, mu2: f64) -> f64 {
        let a = 1.0 + 2.0 / k;
        let b = 1.0 + 1.0 / k;
        let h = gamma(a) / (gamma(b) * gamma(b));
        sigma2 / mu2 - h + 1.0
    }

    // calculate weibull shape and scale from mean and standard deviation
    fn calculate_coefficients(exp: u8, std_dev: u8) -> Result<(f64, f64), SearchError> {
        let mu = exp as f64;
        let sigma = std_dev as f64;
        let sigma2 = sigma.powi(2);
        let mu2 = mu.powi(2);

        let f_closure = |k: f64| Self::f(k, sigma2, mu2);

        let mut convergency = SimpleConvergency { eps: 1e-12, max_iter: 500 };

        let k_root = find_root_brent(0.5, 500.0, f_closure, &mut convergency)?;

        let lambda = mu / gamma(1.0 + 1.0 / k_root);

        Ok((lambda, k_root))
    }
}

impl DeathCalc for WeibullDeath {
    fn new(exp: u8, std_dev: u8) -> impl DeathCalc {
        let Ok((shape, scale)) = WeibullDeath::calculate_coefficients(exp, std_dev) else { todo!() };
        WeibullDeath {expected_age: exp, distribution: Weibull::new(shape, scale).unwrap()}
    }

    fn get_death_age(&self) -> u8 {
        let mut rng = rand::rng();
        self.distribution.sample(&mut rng) as u8
    }

}
