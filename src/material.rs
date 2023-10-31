use crate::{
    vec3::{Vec3, rnd_f32},
    ray::Ray,
    hittable::HitRecord,
    color::Color,
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
    Dielectric(f32),

}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        use Material::*;
        match self {
            Lambertian(color) => Self::scatter_lambertian(color, rec),
            Metal(color, fuzz) => Self::scatter_metal(color, fuzz, r_in, rec),
            Dielectric(ir) => Self::scatter_dielectric(ir, r_in, rec),
        }
    }

    fn scatter_lambertian(albedo: &Color, rec: &HitRecord) -> (Color, Option<Ray>) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        (*albedo, Some(scattered))
    }
    fn scatter_metal(albedo: &Color, fuzz: &f32, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let reflected = r_in.direction().norm().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + *fuzz*Vec3::random_unit_vector());

        if scattered.direction().dot(&rec.normal) > 0.0 {
            (*albedo, Some(scattered))
        } else {
            (*albedo, None)
        }
    }

    fn scatter_dielectric(ir: &f32, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0/ir} else {*ir};

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

    // Schlick's approximation
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.-ref_idx) / (1.+ref_idx);
        r0 = r0*r0;
        r0 + (1.-r0)*(1.-cosine).powf(5.)
    }
}

