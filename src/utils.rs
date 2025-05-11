use rand::Rng;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn random_f64() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

pub fn random_min_max(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
