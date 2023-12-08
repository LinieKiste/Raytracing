use crate::{
    hittable::{Hittable, HitRecord},
    interval::Interval,
    ray::Ray, vec3::{Point3, Vec3},
    material::Material,
    aabb::AABB,
};

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f32,
    w: Vec3,

    mat: Material,
    bbox: AABB
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let bbox = AABB::from_points(q, q+u+v).pad();
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Quad { q, u, v, normal, d, mat, bbox, w }
    }

    fn valid_uv_coords(u: f32, v: f32) -> bool {
        (0.0..1.0).contains(&u) && (0.0..1.0).contains(&v)
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.direction());

        // no hit if parallel to the plane
        if denom.abs() < 1e-8 { return None }

        let t = (self.d - self.normal.dot(&r.origin())) / denom;
        // no hit if intersection outside viable range
        if !ray_t.contains(t) { return None }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !Quad::valid_uv_coords(alpha, beta) {
            None
        } else {
            Some(HitRecord::new(intersection, self.normal, t, r, Some(self.mat.clone()), (alpha, beta)))
        }
    }

    fn bounding_box(&self) -> AABB { self.bbox }
}

