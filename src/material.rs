use std::sync::Arc;
use crate::{
    vec3::{Vec3, rnd_f32},
    ray::Ray,
    hittable::HitRecord,
    color::Color,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new_arc(albedo: Color) -> Arc<Self> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        (self.albedo, Some(scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
impl Metal {
    pub fn new_arc(albedo: Color, fuzz: f32) -> Arc<Self> {
        Arc::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let reflected = r_in.direction().norm().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_unit_vector());

        if scattered.direction().dot(&rec.normal) > 0.0 {
            (self.albedo, Some(scattered))
        } else {
            (self.albedo, None)
        }
    }
}

pub struct Dielectric {
    ir: f32,
}
impl Dielectric {
    pub fn new_arc(ir: f32) -> Arc<Self> {
        Arc::new(Dielectric { ir })
    }

    // Schlick's approximation
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.-ref_idx) / (1.+ref_idx);
        r0 = r0*r0;
        r0 + (1.-r0)*(1.-cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};

        let unit_direction = r_in.direction().norm();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = match cannot_refract ||  Self::reflectance(cos_theta, refraction_ratio) > rnd_f32() {
            true => unit_direction.reflect(&rec.normal),
            false => unit_direction.refract(&rec.normal, refraction_ratio)
        };

        (attenuation, Some(Ray::new(rec.p, direction)))
    }
}

