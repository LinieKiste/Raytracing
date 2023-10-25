mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;

use camera::Camera;
use ray::Ray;
use hittable_list::HittableList;

fn main() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();
    world.main_scene();

    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let mut cam: Camera = Camera::new(aspect_ratio, image_width);
    cam.samples_per_pixel = 100;
    cam.max_bounces = 10;

    cam.render(world)?;

    Ok(())
}

