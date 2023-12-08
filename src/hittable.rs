use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
    interval::Interval,
    material::Material,
    aabb::AABB, sphere::Sphere,
    quad::Quad, triangle::{Triangle, Mesh},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub t: f32,
    pub uv: (f32, f32),
    pub front_face: bool,
}

#[derive(Clone)]
pub enum Primitive {
    Sphere(Sphere),
    Quad(Quad),
    Triangle(Triangle),
    Mesh(Mesh),
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f32, r: &Ray,
               material: Option<Material>, uv: (f32, f32)) -> Self {
        let front_face = r.direction().dot(&outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
            uv,
        }
    }

    pub fn with_material(self, mat: Material) -> Self {
        Self { material: Some(mat), ..self }
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
            Mesh(m) => m.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> AABB {
        match self {
            Sphere(sp) => sp.bounding_box(),
            Quad(q) => q.bounding_box(),
            Triangle(t) => t.bounding_box(),
            Mesh(m) => m.bounding_box(),
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
impl From<Mesh> for Primitive {
    fn from(value: Mesh) -> Self {
        Mesh(value)
    }
}

