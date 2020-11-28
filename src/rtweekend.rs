use rand::Rng;

// Constants

pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees.to_radians();
}

pub fn random_unit_double() -> f64 {
    // Returns a random real in [0,1).
    random_double(0.0, 1.0)
}

pub fn random_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    rand::thread_rng().gen_range(min, max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
