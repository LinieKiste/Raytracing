use crate::{interval::Interval, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_aabbs(box0: &AABB, box1: &AABB) -> Self {
        AABB {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));

        Self { x, y, z }
    }
    pub fn from_3_points(a: Point3, b: Point3, c: Point3) -> Self {
        let x = Interval::new(a.x.min(b.x).min(c.x), a.x.max(b.x).max(c.x));
        let y = Interval::new(a.y.min(b.y).min(c.y), a.y.max(b.y).max(c.y));
        let z = Interval::new(a.z.min(b.z).min(c.z), a.z.max(b.z).max(c.z));

        Self { x, y, z }
    }

    pub fn axis(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn pad(self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {self.x} else {self.x.expand(delta)};
        let new_y = if self.y.size() >= delta {self.y} else {self.y.expand(delta)};
        let new_z = if self.z.size() >= delta {self.z} else {self.z.expand(delta)};

        AABB::new(new_x, new_y, new_z)
    }
}

