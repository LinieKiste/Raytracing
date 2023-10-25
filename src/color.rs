use std::sync::{Arc, Mutex};
use image::ImageBuffer;

use crate::vec3;

pub type Color<T=f32> = vec3::Vec3<T>;

fn linear_to_gamma(color: Color) -> Color {
    color.map(f32::sqrt)
}

pub fn write_color(i: u32, j: u32, buf: Arc<Mutex<ImageBuffer<image::Rgb<u8>, Vec<u8>>>>, color: Color) {
    if let Ok(mut buf) = buf.lock() {
        let color = linear_to_gamma(color);

        let pixel = buf.get_pixel_mut(i as u32, j as u32);
        *pixel = image::Rgb([
                            (256.*color.x) as u8,
                            (256.*color.y) as u8,
                            (256.*color.z) as u8,
        ]);
    }
}

