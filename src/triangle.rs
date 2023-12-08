use std::sync::Arc;
use anyhow::Result;
use crate::{
    hittable::{Hittable, HitRecord},
    interval::Interval,
    ray::Ray,
    material::Material,
    aabb::AABB,
    vec3::{Vec3, Point3}, obj::Obj, BvhNode,
};

const BACKFACE_CULLING: bool = true;

#[derive(Clone)]
pub struct Mesh {
    triangles: Arc<BvhNode>,
    bbox: AABB,
    mat: Material,
}

impl<'a> Mesh {
    pub fn load(filepath: &str) -> Result<Mesh> {
        let obj = Obj::new(filepath)?;

        let mut triangles = vec!();
        let mut bbox = AABB::default();
        for face in &obj.faces {
            let [i0, i1, i2] = face.indices;
            let v0 = obj.vertices[i0 as usize];
            let v1 = obj.vertices[i1 as usize];
            let v2 = obj.vertices[i2 as usize];

            let triangle = Triangle::new(v0, v1, v2);
            bbox = AABB::from_aabbs(&triangle.bounding_box(), &bbox);
            triangles.push(triangle);
        }
        let bvh = BvhNode::new(&mut triangles.into());
        let mat = Material::default();

        Ok(Mesh { triangles: Arc::new(bvh), bbox, mat })
    }

    pub fn new_triangle(t1: Point3, t2: Point3, t3: Point3, mat: Option<Material>) -> Self{
        let tri = Triangle::new(t1, t2, t3);
        let bbox = tri.bounding_box();
        let triangles = Arc::new(BvhNode::new(&mut vec![tri].into()));
        let mat = match mat {
            Some(mat) => mat,
            None => Material::default(),
        };

        Mesh { triangles , bbox, mat }
    }

    pub fn with_material(self, mat: Material) -> Self {
        let mut new = self;
        new.mat = mat;
        new
    }
}

impl<'a> Hittable for Mesh {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.triangles.hit(r, ray_t)
            .map(|h| h.with_material(self.mat.clone()))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Clone)]
pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,

    bbox: AABB,
}

impl Triangle {
    fn new(v0: Point3, v1: Point3, v2: Point3) -> Triangle {
        let bbox = AABB::from_3_points(v0, v1, v2).pad();
        let n = (v1-v0).cross(&(v2-v0));
        let normal = n.normalize();

        Triangle { v0, v1, v2, normal, bbox }
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

        Some(HitRecord::new(intersection, self.normal, t, r, None, (u, v)))
    }

    fn bounding_box(&self) -> AABB { self.bbox }
}

