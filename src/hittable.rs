use crate::{ray::Ray, vec3::{Point3, Vec3}, interval::Interval};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, r: &Ray) -> Self {
        let front_face = r.direction().dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };

        HitRecord {
            p,
            normal,
            t,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

