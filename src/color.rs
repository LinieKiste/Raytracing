use image::ImageBuffer;
use crate::vec3;

pub type Color = vec3::Vec3;

pub fn linear_to_gamma(color: Color) -> Color {
    color.map(f32::sqrt)
}

pub fn write_color(i: u32, j: u32, buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, color: Color) {
    let color = linear_to_gamma(color);

    let pixel = buf.get_pixel_mut(i, j);
    *pixel = image::Rgb([
                        (256.*color.x) as u8,
                        (256.*color.y) as u8,
                        (256.*color.z) as u8,
    ]);
}

