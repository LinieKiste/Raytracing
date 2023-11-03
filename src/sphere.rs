use crate::{
    hittable::*,
    interval::Interval,
    vec3::{Point3, Vec3},
    ray::Ray,
    material::Material,
    aabb::AABB,
};

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self
    {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center-rvec, center+rvec);

        Sphere { center, radius, material, bbox}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().norm_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

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
        let outward_normal = (p - self.center)/self.radius;
        let new_rec = HitRecord::new(p, outward_normal, t, r, self.material);

        Some(new_rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

