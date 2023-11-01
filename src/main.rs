mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;
mod aabb;
mod bvh;
mod quad;

extern crate sdl2;

use camera::Camera;
use ray::Ray;
use hittable_list::HittableList;
use bvh::BvhNode;
use sdl2::{Sdl, render::WindowCanvas};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    // Camera
    let aspect_ratio = 16.0/9.0;
    let image_width = 1600;
    let mut cam: Camera = Camera::new(aspect_ratio, image_width);
    cam.samples_per_pixel = 64;
    cam.max_bounces = 10;

    // World
    let mut world = HittableList::new();
    world.quads(&mut cam);
    let world = BvhNode::new(&mut world);

    // init SDL
    let (sdl_context, canvas) = setup_sdl(&cam)?;

    cam.render(world, canvas, sdl_context)?;

    Ok(())
}

fn setup_sdl(cam: &Camera) -> Result<(Sdl, WindowCanvas)> {
    let sdl_context = sdl2::init()
        .map_err(|e| anyhow!(e))?;
    let video_subsystem = sdl_context.video()
        .map_err(|e| anyhow!(e))?;

    let window = video_subsystem
        .window("Raytracer", cam.image_width, cam.image_height)
        .build()?;
    let canvas = window.into_canvas().build()?;
    Ok((sdl_context, canvas))
}

