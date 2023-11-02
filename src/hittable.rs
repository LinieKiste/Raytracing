use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
    interval::Interval,
    material::Material,
    aabb::AABB, sphere::Sphere, quad::Quad, triangle::Triangle,
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material
}

#[derive(Clone, Copy)]
pub enum Primitive {
    Sphere(Sphere),
    Quad(Quad),
    Triangle(Triangle),
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f32, r: &Ray,
               material: Material) -> Self {
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
    fn bounding_box(&self) -> AABB;
}

use Primitive::*;

impl Hittable for Primitive {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            Sphere(sp) => sp.hit(r, ray_t),
            Quad(q) => q.hit(r, ray_t),
            Triangle(t) => t.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Sphere(sp) => sp.bounding_box(),
            Quad(q) => q.bounding_box(),
            Triangle(t) => t.bounding_box(),
        }
    }
}

impl From<Sphere> for Primitive {
    fn from(value: Sphere) -> Self {
        Sphere(value)
    }
}
impl From<Quad> for Primitive {
    fn from(value: Quad) -> Self {
        Quad(value)
    }
}
impl From<Triangle> for Primitive {
    fn from(value: Triangle) -> Self {
        Triangle(value)
    }
}

