use crate::{
    vec3::{Vec3, reflect, refract},
    ray::Ray,
    hittable::HitRecord,
    color::Color,
    texture::Texture,
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Texture),
    Metal(Color, f32),
    Dielectric(f32),

}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        use Material::*;
        match self {
            Lambertian(texture) => Self::scatter_lambertian(texture, rec),
            Metal(color, fuzz) => Self::scatter_metal(color, fuzz, r_in, rec),
            Dielectric(ir) => Self::scatter_dielectric(ir, r_in, rec),
        }
    }

    fn scatter_lambertian(albedo: &Texture, rec: &HitRecord) -> (Color, Option<Ray>) {
        let mut scatter_direction = rec.normal + Vec3::new_random();
        if scatter_direction.relative_eq(&Vec3::zeros(), 0.001, 0.1) {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        (albedo.value(rec.uv, rec.p), Some(scattered))
    }
    fn scatter_metal(albedo: &Color, fuzz: &f32, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let reflected = reflect(r_in.direction().normalize(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + *fuzz*Vec3::new_random());

        if scattered.direction().dot(&rec.normal) > 0.0 {
            (*albedo, Some(scattered))
        } else {
            (*albedo, None)
        }
    }

    fn scatter_dielectric(ir: &f32, r_in: &Ray, rec: &HitRecord) -> (Color, Option<Ray>) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {1.0/ir} else {*ir};

        let unit_direction = r_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = match cannot_refract ||  Self::reflectance(cos_theta, refraction_ratio) > rand::random() {
            true => reflect(unit_direction, &rec.normal),
            false => refract(unit_direction, &rec.normal, refraction_ratio)
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

impl Default for Material {
    fn default() -> Self {
        let pink = Texture::new_solid_rgb(1., 0., 220./255.);
        let black = Texture::new_solid_rgb(0., 0., 0.);
        let checkered = Texture::new_checkered(1., pink, black);
        Self::Lambertian(checkered)
    }
}

impl From<Texture> for Material {
    fn from(value: Texture) -> Self {
        Self::Lambertian(value)
    }
}

impl From<&Texture> for Material {
    fn from(value: &Texture) -> Self {
        Self::Lambertian(value.clone())
    }
}

