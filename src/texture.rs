use std::sync::Arc;
use anyhow::Result;
use crate::{
    color::Color,
    vec3::Point3
};

#[derive(Clone)]
pub struct CheckerTexture {
    scale: f32,
    even: Texture,
    odd: Texture,
}

#[derive(Clone)]
pub enum Texture {
    SolidColor(Color),
    CheckerTexture(Arc<CheckerTexture>),
    ImageTexture(Option<Arc<RgbImage>>),
}

use Texture::*;
use image::RgbImage;
impl Texture {
    pub fn new_solid(col: Color) -> Self {
        SolidColor(col)
    }
    pub fn new_solid_rgb(r: f32, g: f32, b: f32) -> Self {
        SolidColor(Color::new(r, g, b))
    }
    pub fn new_checkered(scale: f32, even: Texture, odd: Texture) -> Self {
        let ct = CheckerTexture {
            scale,
            even,
            odd
        };
        CheckerTexture(Arc::new(ct))
    }
    pub fn new_image(path: &str) -> Self {
        if let Ok(image) = image::open(path) {
            ImageTexture(Some(Arc::new(image.into_rgb8())))
        } else {
            ImageTexture(None)
        }
    }

    pub fn value(&self, uv: (f32, f32), p: Point3) -> Color {
        match *self {
            SolidColor(c) => c,
            CheckerTexture(ref c) => {
                let x_int = (&c.scale * p.x).floor() as i32;
                let y_int = (&c.scale * p.y).floor() as i32;
                let z_int = (&c.scale * p.z).floor() as i32;
                let is_even: bool = (x_int + y_int + z_int) % 2 == 0;

                if is_even { c.even.value(uv, p) } else { c.odd.value(uv, p) }
            },
            ImageTexture(ref image) => {
                let Some(img) = image else {return Color::new(1.,0.,1.)};

                let uv = (uv.0.clamp(0.0, 1.0), 1. - uv.1.clamp(0.0, 1.0));

                let x = (uv.0 * img.width() as f32) as u32;
                let y = (uv.1 * img.height() as f32) as u32;
                let pixel = img.get_pixel(x, y);

                let color_scale = 1.0/255.0;
                Color::new(
                    color_scale*pixel[0] as f32,
                    color_scale*pixel[1] as f32,
                    color_scale*pixel[2] as f32
                    )
            },
        }
    }
}

impl From<Color> for Texture {
    fn from(value: Color) -> Self {
        Texture::new_solid(value)
    }
}

