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
        Metal,
        Emissive,
    },
    aabb::AABB, quad::Quad, triangle::Mesh, texture::Texture,
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
    pub fn random_spheres(&mut self, cam: &mut Camera) {
        let green = Color::new(0.2, 0.3, 0.1).into();
        let white = Color::new(0.9, 0.9, 0.9).into();
        let checkered = Texture::new_checkered(0.32, green, white);
        let ground_material = Material::Lambertian(checkered);
        self.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f32 = random();
                let center =Point3::new(a as f32 + 0.9*random::<f32>(), 0.2,
                    b as f32+0.9*random::<f32>());

                if (center-Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                    let sphere_material = if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::new_random().component_mul(&Color::new_random());
                        Lambertian(albedo.into())
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = (Color::new_random()/2.0).add_scalar(0.5);
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

        let mat2 = Lambertian(Color::new(0.4, 0.2, 0.1).into());
        self.add(Sphere::new(Point3::new(-4.,1.,0.), 1.0, mat2));

        let mat3 = Metal(Color::new(0.7, 0.6, 0.5), 0.0);
        self.add(Sphere::new(Point3::new(4.,1.,0.), 1.0, mat3));

        cam.fov = 20.;
        cam.lookfrom = Point3::new(13.0,2.0,3.);
        cam.lookat   = Point3::new(0.,0.,0.);
    }

    pub fn two_spheres(&mut self, cam: &mut Camera) {
        let even = Color::new(0.2, 0.3, 0.1).into();
        let odd = Color::new(0.9, 0.9, 0.9).into();
        let checker = Texture::new_checkered(0.8, even, odd);

        self.add(Sphere::new(Point3::new(0., -10., 0.), 10., checker.clone().into()));
        self.add(Sphere::new(Point3::new(0., 10., 0.), 10., checker.into()));

        cam.fov = 20.;
        cam.lookfrom = Point3::new(13.,2.,3.);
        cam.lookat = Point3::zeros();
    }

    pub fn earth(&mut self, cam: &mut Camera) {
        let earth_texture = Texture::new_image("assets/blue_marble.jpg");
        let earth_surface = Lambertian(earth_texture);
        let globe = Sphere::new(Point3::zeros(), 2.0, earth_surface);

        self.add(globe);

        cam.fov = 20.;
        cam.lookfrom = Point3::new(12.,8.,-5.);
        cam.lookat = Point3::zeros();
    }

    pub fn quads(&mut self, cam: &mut Camera) {
        // Materials
        let left_red     = Lambertian(Color::new(1.0, 0.2, 0.2).into());
        let back_green   = Lambertian(Color::new(0.2, 1.0, 0.2).into());
        let right_blue   = Lambertian(Color::new(0.2, 0.2, 1.0).into());
        let upper_orange = Lambertian(Color::new(1.0, 0.5, 0.0).into());
        let lower_teal   = Lambertian(Color::new(0.2, 0.8, 0.8).into());

        // Quads
        let y_4 = Vec3::new(0., 4., 0.);
        let tilt = Vec3::new(-2., 4., 0.);
        let x_4 = Vec3::new(4., 0., 0.);
        let z_4 = Vec3::new(0., 0., 4.);
        let z_neg_4 = Vec3::new(0., 0., -4.);
        self.add(Quad::new(Point3::new(-3., -2., 5.), z_neg_4, y_4, left_red));
        let (t1, t2, t3) = (Point3::new(-2., -2., 0.),
                Point3::new(2.,-2.,0.), Point3::new(-2.,2.,0.));
        self.add(Mesh::new_triangle(t1, t2, t3, Some(back_green)));
        // self.add(Triangle::new(Point3::new( 3., -2., 1.), z_4, y_4, right_blue));
        self.add(Quad::new(Point3::new(-2.,  3., 1.), x_4, z_4, upper_orange));
        self.add(Quad::new(Point3::new(-2., -3., 5.), x_4, z_neg_4, lower_teal));

        cam.fov = 80.0;
        cam.lookfrom = Point3::new(0., 0., 9.);
        cam.lookat = Point3::new(0., 0., 0.);
    }
    pub fn triangle_mesh(&mut self, cam: &mut Camera) {
        let blue = Metal(Color::new(0.2, 0.4, 0.9), 0.2);
        let emissive = Emissive(Color::new(0.7, 0.7, 0.7), 10.);
        let mesh = Mesh::load("assets/teapot.obj").unwrap();

        let gray = Texture::new_solid_rgb(0.7, 0.7, 0.7);
        // let boundary = Mesh::load("assets/box.obj").unwrap()
        //     .with_material(gray.into());

        self.add(mesh);
        // self.add(boundary);

        cam.fov = 80.0;
        cam.lookfrom = Point3::new(0., 3., 5.);
        cam.lookat = Point3::new(0., 1., 0.);
    }
    pub fn bugatti(&mut self, cam: &mut Camera) {
        let car = Mesh::load("assets/bugatti/bugatti.obj").unwrap();
        let bg = Mesh::load("assets/bugatti/background.obj").unwrap();

        self.add(car);
        self.add(bg);

        cam.fov = 60.0;
        cam.lookfrom = Point3::new(6., 3., 7.);
        cam.lookat = Point3::new(0., 1., 0.);
    }
    pub fn cornell_box(&mut self, cam: &mut Camera) {
        let red: Material = Texture::new_solid_rgb(0.65, 0.05, 0.05).into();
        let white: Material = Texture::new_solid_rgb(0.73, 0.73, 0.73).into();
        let green: Material = Texture::new_solid_rgb(0.12, 0.45, 0.15).into();
        let light = Emissive(Color::new(1., 1., 1.), 15.);

        let fst_555 = Vec3::new(555., 0., 0.);
        let snd_555 = Vec3::new(0., 555., 0.);
        let third_555 = Vec3::new(0., 0., 555.);
        self.add(Quad::new(fst_555, snd_555, third_555, green));
        self.add(Quad::new(Vec3::zeros(), snd_555, third_555, red));
        self.add(Quad::new(Vec3::new(343., 554., 332.), Vec3::new(-130., 0., 0.),
            Vec3::new(0., 0., -105.), light));
        self.add(Quad::new(Vec3::zeros(), fst_555, third_555, white.clone())); // bottom quad
        self.add(Quad::new(Vec3::new(555., 555., 555.), -fst_555, -third_555, white.clone())); // top quad
        self.add(Quad::new(third_555, fst_555, snd_555, white.clone()));
        // world.add(make_shared<quad>(point3(0,0,555), vec3(555,0,0), vec3(0,555,0), white));

        cam.fov = 40.;
        cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
        cam.lookat = Point3::new(278., 278., 0.0);
        // cam.background = Color::new(0.0, 0.0, 0.0);
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

impl<T: Into<Primitive>> From<Vec<T>> for HittableList<Primitive> {
    fn from(value: Vec<T>) -> Self {
        let mut list = HittableList::new();
        for elem in value {
            list.add(elem);
        }

        list
    }
}

