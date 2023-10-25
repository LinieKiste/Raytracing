use std::sync::{Arc, Mutex};
use crate::color::write_color;
use std::f32::INFINITY;
use std::io::Result;
use image::{Rgb, ImageBuffer};
use rand::Rng;
use rayon::prelude::*;

use crate::Ray;
use crate::interval::Interval;
use crate::vec3::{
    Point3, Vec3
};
use crate::{hittable::Hittable, color::Color};

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub fov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    // public
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);
        let samples_per_pixel = 16;
        let max_bounces = 10;

        let fov: f32 = 20.;
        let lookfrom = Point3::new(13.0,2.0,3.);
        let lookat   = Point3::new(0.,0.,0.);
        let vup      =   Vec3::new(0.,1.,0.);

        // Camera
        let focal_length = (lookfrom - lookat).length();
        let theta = fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height *
            (image_width as f32/image_height as f32);
        let center = lookfrom;

        // base vectors
        let w = (lookfrom - lookat).norm();
        let u = vup.cross(&w).norm();
        let v = w.cross(&u);

        // Viewport vectors
        let viewport_u = viewport_width * u; // horizontal viewport edge
        let viewport_v = viewport_height * -v; // vertical viewport edge

        // Horizontal and vertical delta between pixels
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Upper left pixel
        let viewport_upper_left = center -
            focal_length*w - viewport_u/2. - viewport_v/2.;
        let pixel00_loc = viewport_upper_left +
            0.5 * (pixel_delta_u+pixel_delta_v);

        Camera {
            aspect_ratio, image_width,
            image_height, center, pixel00_loc,
            pixel_delta_u, pixel_delta_v, samples_per_pixel,
            max_bounces, fov, lookfrom, lookat,
            vup, u, v, w,
        }
    }

    pub fn render<T: Hittable + Sync>(&mut self, world: T) -> Result<()> {

        let imgbuf: Arc<Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>>>
            = Arc::new(Mutex::new(ImageBuffer::new(self.image_width, self.image_height)));
        for j in 0..self.image_height {
            eprint!{"\rScanlines remaining: {} ", (self.image_height - j)};

            (0..self.image_width).into_par_iter().for_each(|i| {
                let mut pixel_color = Color::new(0.,0.,0.);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_bounces, &world);
                }
                pixel_color /= self.samples_per_pixel;

                write_color(i, j, imgbuf.clone(), pixel_color);
            })
        }
        imgbuf.lock().unwrap().save("image.png").unwrap();
        eprintln!("\r Done.                   ");

        Ok(())
    }

    // private
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc +
            (i*self.pixel_delta_u) + (j*self.pixel_delta_v);
        let pixel_sample = pixel_center + self.sample_loc();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_loc(&self) -> Vec3 {
        let px = -0.5 + rand::thread_rng().gen::<f32>();
        let py = -0.5 + rand::thread_rng().gen::<f32>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}

fn ray_color<T: Hittable + Sync>(r: &Ray, depth: u32, world: &T) -> Color {
    // If the ray bounce limit has been exceeded, we return black
    if depth <= 0 { return Color::new(0.,0.,0.) }

    if let Some(hit) = world.hit(r, Interval::new(0.001, INFINITY)) {
        if let (attenuation, Some(scattered)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(&scattered, depth-1, world)
        }

        return Color::new(0.,0.,0.);
    }

    let unit_direction = r.direction().norm();
    let a = 0.5*(unit_direction.y + 1.0);

    (1.0-a)*Color::new(1.,1.,1.) + a*Color::new(0.5,0.7,1.0)
}

