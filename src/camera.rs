use anyhow::{Result, anyhow};
use sdl2::EventPump;
use sdl2::{
    event::Event,
    Sdl,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{WindowCanvas, Texture},
};
use crate::color::{write_color, linear_to_gamma};
use std::f32::INFINITY;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{
    Point3, Vec3
};
use crate::{hittable::Hittable, color::Color};

#[allow(dead_code)]
#[derive(Default)]
pub struct Camera {
    aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub fov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,

    imgbuf: RgbImage,
}

impl Camera {
    // public
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);
        let samples_per_pixel = 16;
        let max_bounces = 10;

        let fov: f32 = 80.;
        let lookfrom = Point3::new(9.0,0.0,0.);
        let lookat   = Point3::new(0.,0.,0.);
        let vup      =   Vec3::new(0.,1.,0.);

        let imgbuf: RgbImage = ImageBuffer::new(image_width, image_height);

        Camera {
            aspect_ratio, image_width,
            image_height,
            samples_per_pixel,
            max_bounces, fov, lookfrom, lookat,
            vup, imgbuf, ..Default::default()
        }
    }
    
    fn update(&mut self) {
        let focal_length = (self.lookfrom - self.lookat).norm();
        let theta = self.fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height *
            (self.image_width as f32/self.image_height as f32);
        self.center = self.lookfrom;

        // base vectors
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        // Viewport vectors
        let viewport_u = viewport_width * self.u; // horizontal viewport edge
        let viewport_v = viewport_height * -self.v; // vertical viewport edge

        // Horizontal and vertical delta between pixels
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Upper left pixel
        let viewport_upper_left = self.center -
            focal_length*self.w - viewport_u/2. - viewport_v/2.;
        self.pixel00_loc = viewport_upper_left +
            0.5 * (self.pixel_delta_u+self.pixel_delta_v);
    }

    pub fn render_with_preview<T: Hittable+Sync>(&mut self, world: &T)
        -> Result<()> {

            // init SDL
            let (sdl_context, canvas) = self.setup_sdl()?;

            let mut event_pump = sdl_context
                .event_pump()
                .map_err(|e| anyhow!(e))?;
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_streaming(PixelFormatEnum::RGB24, self.image_width, self.image_height)?;

            // Measure time
            eprintln!("Starting render");
            let start = Instant::now();

            self.preview_render_loop(world, texture, canvas, &mut event_pump)?;

            eprintln!("\r Done.                   ");
            let duration = start.elapsed();
            eprintln!("Render took {:.2?}", duration);

            self.imgbuf.save("image.png").unwrap();

            Ok(())
        }

    pub fn render_no_preview<T: Hittable+Sync>(&mut self, world: &T) -> Result<()> {
        self.update();

        for j in 0..self.image_height {
            eprint!{"\rScanlines remaining: {} ", (self.image_height - j)};

            for i in 0..self.image_width {
                let mut pixel_color = (0..self.samples_per_pixel).into_iter()
                    .map(|_| {
                        let r = self.get_ray(i,j);
                        ray_color(&r, self.max_bounces, world)
                    })
                .sum::<Vec3>();
                pixel_color /= self.samples_per_pixel as f32;

                write_color(i, j, &mut self.imgbuf, pixel_color);
            }
        }
        self.imgbuf.save("image.png").unwrap();
        Ok(())
    }

    // private
    fn preview_render_loop<T: Hittable+Sync>(&mut self, world: &T, mut texture: Texture<'_>, mut canvas: WindowCanvas, event_pump: &mut EventPump)
        -> Result<()> {
        self.update();

        'rendering: {
            for j in 0..self.image_height {
                eprint!{"\rScanlines remaining: {} ", (self.image_height - j)};

                texture.with_lock(None, |buffer, pitch| {
                    for i in 0..self.image_width {
                        let mut pixel_color = (0..self.samples_per_pixel).into_iter()
                            .map(|_| {
                                let r = self.get_ray(i,j);
                                ray_color(&r, self.max_bounces, world)
                            })
                        .sum::<Vec3>();
                        pixel_color /= self.samples_per_pixel as f32;

                        write_color(i, j, &mut self.imgbuf, pixel_color);

                        Self::write_to_buffer(i, j, buffer, pitch, pixel_color);
                    }
                })
                .map_err(|e| anyhow!(e))?;
                canvas.clear();
                canvas.copy(&texture, None, None).map_err(|e| anyhow!(e))?;
                canvas.present();
                if Self::poll_quit(event_pump) {
                    eprintln!(); break 'rendering
                }
            }
            loop { if Self::poll_quit(event_pump) { break 'rendering }}
        }

        Ok(())
    }

    fn setup_sdl(&self) -> Result<(Sdl, WindowCanvas)> {
        let sdl_context = sdl2::init()
            .map_err(|e| anyhow!(e))?;
        let video_subsystem = sdl_context.video()
            .map_err(|e| anyhow!(e))?;

        let window = video_subsystem
            .window("Raytracer", self.image_width, self.image_height)
            .build()?;
        let canvas = window.into_canvas().build()?;
        Ok((sdl_context, canvas))
    }

    fn poll_quit(event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                | Event::KeyDown { keycode: Some(Keycode::Q), ..}
                => return true,
                _ => {}
            }
        }
        false
    }
    fn write_to_buffer(i: u32, j: u32, buffer: &mut [u8], pitch: usize, color: Color) {
        let color = linear_to_gamma(color);
        let offset: usize = (j*pitch as u32 + i*3) as usize;

        buffer[offset] = (256.*color.x) as u8;
        buffer[offset + 1] = (256.*color.y) as u8;
        buffer[offset + 2] = (256.*color.z) as u8;
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc +
            (i as f32*self.pixel_delta_u) + (j as f32*self.pixel_delta_v);
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
    if depth == 0 { return Color::new(0.,0.,0.) }

    if let Some(hit) = world.hit(r, Interval::new(0.001, INFINITY)) {
        if let (attenuation, Some(scattered)) = hit.material.scatter(r, &hit) {
            return attenuation.component_mul(&ray_color(&scattered, depth-1, world));
        }

        return Color::new(0.,0.,0.);
    }

    let unit_direction = r.direction().normalize();
    let a = 0.5*(unit_direction.y + 1.0);

    (1.0-a)*Color::new(1.,1.,1.) + a*Color::new(0.5,0.7,1.0)
}

