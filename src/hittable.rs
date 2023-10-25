use std::sync::Arc;

use crate::{ray::Ray, vec3::{Point3, Vec3}, interval::Interval, material::Material};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material + Sync + Send>
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f32, r: &Ray,
               material: Arc<dyn Material+Sync+Send>) -> Self {
        let front_face = r.direction().dot(&outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

