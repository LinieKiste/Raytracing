use std::f32::{INFINITY, NEG_INFINITY};

const EMPTY: Interval = Interval { min: INFINITY, max: NEG_INFINITY };
const UNIVERSE: Interval = Interval { min: NEG_INFINITY, max: INFINITY };

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { self.min }
        else if x > self.max { self.max }
        else { x }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }
}

