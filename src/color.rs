use crate::vec3;
use std::io::Write;
use std::fs::File;

pub type Color<T=f32> = vec3::Vec3<T>;

pub fn write_color(out: &mut File, pixel_color: Color){
    write!(out, 
           "{} {} {}\n",
           (255.999*pixel_color.x) as u32,
           (255.999*pixel_color.y) as u32,
           (255.999*pixel_color.z) as u32,
           )
        .expect("Error writing color!")
}

