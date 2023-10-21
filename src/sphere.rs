use crate::hittable::*;
use crate::interval::Interval;
use crate::vec3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. { return None; }
        let sqrtd = f32::sqrt(discriminant);

        // Find nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let normal = (p - self.center)/self.radius;
        let new_rec = HitRecord::new(p, normal, t, r);

        Some(new_rec)
    }
}

