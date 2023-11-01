use crate::{vec3::{Vec3, Point3}, interval::Interval, aabb::AABB};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray{orig, dir}
    }
    pub fn origin(&self) -> Point3 { self.orig }
    pub fn direction(&self) -> Vec3 { self.dir }
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t*self.dir
    }
}

pub trait Intersect<T> {
    fn intersects(self, other: &T, range: Interval) -> bool;
}

impl Intersect<AABB> for &Ray {
    fn intersects(self, other: &AABB, range: Interval) -> bool {
        let mut ray_t = range;
        for a in 0..3 {
            let inv_d = 1./self.direction()[a];
            let orig = self.origin()[a];
            
            let mut t0 = (other.axis(a).min - orig) * inv_d;
            let mut t1 = (other.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 { (t0, t1) = (t1, t0) }

            if t0 > ray_t.min { ray_t.min = t0; }
            if t1 < ray_t.max { ray_t.max = t1; }

            if ray_t.max <= ray_t.min { return false; }
        }
        true
    }
}

