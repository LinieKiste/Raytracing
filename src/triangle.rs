use crate::{
    hittable::{Hittable, HitRecord},
    interval::Interval,
    ray::Ray,
    material::Material,
    aabb::AABB,
    vec3::{Vec3, Point3},
};

const BACKFACE_CULLING: bool = true;

#[derive(Clone)]
pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,

    mat: Material,
    bbox: AABB
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: Material) -> Self {
        let bbox = AABB::from_3_points(v0, v1, v2).pad();
        let n = (v1-v0).cross(&(v2-v0));
        let normal = n.normalize();

        Triangle { v0, v1, v2, normal, mat, bbox }
    }
}

impl Hittable for Triangle {
    // Möller-Trumbore algorithm
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let v0v1 = self.v1-self.v0;
        let v0v2 = self.v2-self.v0;
        let pvec = r.direction().cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if BACKFACE_CULLING && det < f32::EPSILON { return None }
        if det.abs() < f32::EPSILON { return None }

        let inv_det = 1./det;

        let tvec = r.origin() - self.v0;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0. || u > 1. { return None }

        let qvec = tvec.cross(&v0v1);
        let v = r.direction().dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. { return None }

        let t = v0v2.dot(&qvec) * inv_det;
        // no hit if intersection outside viable range
        if !ray_t.contains(t) { return None }

        let intersection = r.at(t);
        // let planar_hitpt_vector = intersection - self.v0;

        Some(HitRecord::new(intersection, self.normal, t, r, self.mat.clone(), (u, v)))
    }

    fn bounding_box(&self) -> AABB { self.bbox }
}

