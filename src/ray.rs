use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray{orig, dir}
    }
    pub fn origin(&self) -> Point3 { self.orig.clone() }
    pub fn direction(&self) -> Vec3 { self.dir.clone() }
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t*self.dir
    }
}
