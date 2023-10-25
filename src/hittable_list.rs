use std::sync::Arc;

use crate::material::{Dielectric, Material, Metal};
use rand::{random, Rng};

use crate::{ray::Ray, hittable::{Hittable, HitRecord}, interval::Interval, material::Lambertian, color::Color, sphere::Sphere, vec3::Point3};

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        HittableList { objects: vec!() }
    }

    pub fn add(&mut self, item: T) {
        self.objects.push(item)
    }

}
impl HittableList<Sphere> {
    pub fn main_scene(&mut self) {
        let ground_material = Lambertian::new_arc(Color::new(0.5, 0.5, 0.5));
        self.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = random();
                let center =Point3::new(a as f32 + 0.9*random::<f32>(), 0.2,
                    b as f32+0.9*random::<f32>());

                if (center-Point3::new(4., 0.2, 0.)).length() > 0.9 {
                    let sphere_material: Arc<dyn Material+Send+Sync> = if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::rnd() * Color::rnd();
                        Lambertian::new_arc(albedo)
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Color::rnd_range(0.5..1.0);
                        let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                        Metal::new_arc(albedo, fuzz)
                    } else {
                        // glass
                        Dielectric::new_arc(1.5)
                    };
                    self.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }

        let mat1 = Dielectric::new_arc(1.5);
        self.add(Sphere::new(Point3::new(0.,1.,0.), 1.0, mat1));

        let mat2 = Lambertian::new_arc(Color::new(0.4, 0.2, 0.1));
        self.add(Sphere::new(Point3::new(-4.,1.,0.), 1.0, mat2));

        let mat3 = Metal::new_arc(Color::new(0.7, 0.6, 0.5), 0.0);
        self.add(Sphere::new(Point3::new(4.,1.,0.), 1.0, mat3));
    }
}

impl<'a, T: Hittable + Sync + Send> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}

