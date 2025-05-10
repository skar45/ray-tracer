use crate::utils::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        (self.min <= x) && (self.max >= x)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        (self.min < x) && (self.max > x)
    }

    pub fn clamp(&self, x:f64) -> f64 {
        if x < self.min { return self.min };
        if x > self.max { return self.max };
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
