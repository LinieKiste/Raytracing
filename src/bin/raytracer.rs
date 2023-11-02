use raytracing::camera::Camera;
use raytracing::hittable_list::HittableList;
use raytracing::bvh::BvhNode;
use anyhow::Result;

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

    cam.render_with_preview(&world)?;

    Ok(())
}

