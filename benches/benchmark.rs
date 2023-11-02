use criterion::{criterion_group, criterion_main, Criterion};
use raytracing::{
    Camera,
    HittableList,
    BvhNode,
};

fn quads_and_spheres(c: &mut Criterion) {
    let aspect_ratio = 16.0/9.0;
    let image_width = 100;
    let mut cam = Camera::new(aspect_ratio, image_width);
    cam.samples_per_pixel = 5;
    cam.max_bounces = 5;

    // World
    let mut world = HittableList::new();
    world.quads(&mut cam);

    c.bench_function("Quads without BVH", |b| b.iter(||{
        let _ = cam.render_no_preview(&world);
    }));
    c.bench_function("Quads with BVH", |b| b.iter(||{
        let world = BvhNode::new(&mut world);
        let _ = cam.render_no_preview(&world);
    }));

    let mut world = HittableList::new();
    world.random_spheres(&mut cam);
    c.bench_function("Spheres without BVH", |b| b.iter(||{
        let _ = cam.render_no_preview(&world);
    }));
    c.bench_function("Spheres with BVH", |b| b.iter(||{
        let world = BvhNode::new(&mut world);
        let _ = cam.render_no_preview(&world);
    }));
}

criterion_group!(benches, quads_and_spheres);
criterion_main!(benches);

