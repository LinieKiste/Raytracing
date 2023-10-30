mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;

extern crate sdl2;

use camera::Camera;
use ray::Ray;
use hittable_list::HittableList;

fn main() -> anyhow::Result<(), String> {

    // World
    let mut world = HittableList::new();
    world.main_scene();

    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let mut cam: Camera = Camera::new(aspect_ratio, image_width);
    cam.samples_per_pixel = 32;
    cam.max_bounces = 10;

    // init SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Raytracer", cam.image_width, cam.image_height)
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    cam.render(world, canvas, sdl_context).map_err(|e| e.to_string())?;

    Ok(())
}



