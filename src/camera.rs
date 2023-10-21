use crate::ray_color;
use std::f32::INFINITY;
use std::io::Result;

use crate::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::{hittable::Hittable, color::Color};

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    // public
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height < 1 {1} else {image_height};
        let center = Point3::new(0.,0.,0.);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height *
            (image_width as f32/image_height as f32);
        let camera_center = Point3::new(0.,0.,0.);

        // Viewport vectors
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Horizontal and vertical delta between pixels
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Upper left pixel
        let viewport_upper_left: Vec3 = camera_center -
            Vec3::new(0.,0.,focal_length) - viewport_u/2. - viewport_v/2.;
        let pixel00_loc = viewport_upper_left +
            0.5 * (pixel_delta_u+pixel_delta_v);

        Camera {
            aspect_ratio, image_width,
            image_height, center, pixel00_loc,
            pixel_delta_u, pixel_delta_v
        }
    }

    pub fn render(&mut self, world: impl Hittable) -> Result<()> {

        let mut imgbuf = image::ImageBuffer::new(self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!{"\rScanlines remaining: {} ", (self.image_height - j)};
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc +
                    (i*self.pixel_delta_u) + (j*self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = ray_color(&r, &world);

                let pixel = imgbuf.get_pixel_mut(i as u32, j as u32);
                *pixel = image::Rgb([
                                    (255.999*pixel_color.x) as u8,
                                    (255.999*pixel_color.y) as u8,
                                    (255.999*pixel_color.z) as u8,
                ]);
            }
        }
        imgbuf.save("image.png").unwrap();
        eprintln!("\r Done.                   ");

        Ok(())
    }

    // private
    fn ray_color(r: &Ray, world: impl Hittable) -> Color {
        if let Some(hit) = world.hit(r, Interval::new(0., INFINITY)) {
            return 0.5 * (hit.normal + Color::new(1.,1.,1.));
        }

        let unit_direction = r.direction().norm();
        let a = 0.5*(unit_direction.y + 1.0);

        (1.0-a)*Color::new(1.,1.,1.) + a*Color::new(0.5,0.7,1.0)
    }
}

