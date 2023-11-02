use rand::{random, Rng};

use crate::{
    camera::Camera,
    ray::Ray,
    hittable::{
        Hittable, HitRecord,
        Primitive
    },
    interval::Interval,
    color::Color,
    sphere::Sphere,
    vec3::{Point3, Vec3},
    material::Material::{
        self,
        Lambertian,
        Dielectric,
        Metal
    },
    aabb::AABB, quad::Quad, triangle::Triangle,
};

pub struct HittableList<T: Hittable> {
    pub objects: Vec<T>,
    bbox: AABB,
}

impl HittableList<Primitive> {
    pub fn new() -> Self {
        HittableList { objects: vec!(), bbox: Default::default() }
    }

    pub fn add<T: Into<Primitive>>(&mut self, item: T) {
        let item = item.into();
        self.bbox = AABB::from_aabbs(&self.bbox, &item.bounding_box());
        self.objects.push(item)
    }
}

impl HittableList<Primitive> {
    pub fn quads(&mut self, cam: &mut Camera) {
        // Materials
        let left_red     = Lambertian(Color::new(1.0, 0.2, 0.2));
        let back_green   = Lambertian(Color::new(0.2, 1.0, 0.2));
        let right_blue   = Lambertian(Color::new(0.2, 0.2, 1.0));
        let upper_orange = Lambertian(Color::new(1.0, 0.5, 0.0));
        let lower_teal   = Lambertian(Color::new(0.2, 0.8, 0.8));

        // Quads
        let y_4 = Vec3::new(0., 4., 0.);
        let tilt = Vec3::new(-2., 4., 0.);
        let x_4 = Vec3::new(4., 0., 0.);
        let z_4 = Vec3::new(0., 0., 4.);
        let z_neg_4 = Vec3::new(0., 0., -4.);
        self.add(Quad::new(Point3::new(-3., -2., 5.), z_neg_4, y_4, left_red));
        self.add(Triangle::new(Point3::new(-2., -2., 0.), x_4, tilt, back_green));
        self.add(Triangle::new(Point3::new( 3., -2., 1.), z_4, y_4, right_blue));
        self.add(Quad::new(Point3::new(-2.,  3., 1.), x_4, z_4, upper_orange));
        self.add(Quad::new(Point3::new(-2., -3., 5.), x_4, z_neg_4, lower_teal));

        cam.fov = 80.0;
        cam.lookfrom = Point3::new(0., 0., 9.);
        cam.lookat = Point3::new(0., 0., 0.);
    }

    pub fn random_spheres(&mut self, cam: &mut Camera) {
        let ground_material = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
        self.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = random();
                let center =Point3::new(a as f32 + 0.9*random::<f32>(), 0.2,
                    b as f32+0.9*random::<f32>());

                if (center-Point3::new(4., 0.2, 0.)).length() > 0.9 {
                    let sphere_material = if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::rnd() * Color::rnd();
                        Lambertian(albedo)
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Color::rnd_range(0.5..1.0);
                        let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                        Metal(albedo, fuzz)
                    } else {
                        // glass
                        Dielectric(1.5)
                    };
                    self.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }

        let mat1 = Dielectric(1.5);
        self.add(Sphere::new(Point3::new(0.,1.,0.), 1.0, mat1));

        let mat2 = Lambertian(Color::new(0.4, 0.2, 0.1));
        self.add(Sphere::new(Point3::new(-4.,1.,0.), 1.0, mat2));

        let mat3 = Metal(Color::new(0.7, 0.6, 0.5), 0.0);
        self.add(Sphere::new(Point3::new(4.,1.,0.), 1.0, mat3));

        cam.fov = 20.;
        cam.lookfrom = Point3::new(13.0,2.0,3.);
        cam.lookat   = Point3::new(0.,0.,0.);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

