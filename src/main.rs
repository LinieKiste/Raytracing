mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;

use camera::Camera;
use vec3::Point3;
use ray::Ray;
use sphere::Sphere;
use hittable_list::HittableList;

fn main() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.,0.,-1.), 0.5));
    world.add(Sphere::new(Point3::new(0.,-100.5,-1.), 100.));

    let aspect_ratio = 16.0/9.0;
    let image_width = 800;
    let mut cam: Camera = Camera::new(aspect_ratio, image_width);
    cam.samples_per_pixel = 32;
    cam.max_bounces = 10;

    cam.render(world)?;

    Ok(())
}

