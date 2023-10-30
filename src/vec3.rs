use core::fmt;
use std::ops::{self, Range};
use std::iter::Sum;

use rand::Rng;

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T=f32> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 {x, y, z}
    }
    pub fn map<U, F>(self, mut f: F) -> Vec3<U>
    where
        F: FnMut(T) -> U
    {
        let Vec3{x, y, z} = self;
        Vec3 {
            x: f(x),
            y: f(y),
            z: f(z)
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(0.,0.,0.), |a, b| {
            a+b
        })
    }
}

macro_rules! impl_op_for_vec {
    ($op:path, $out_type:path, $op_generic:path, $fname:ident) => {
        impl<T> $op for Vec3<T>
            where T: $out_type,
        {
            type Output = Vec3<T>;
            fn $fname(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$fname(rhs.x),
                    y: self.y.$fname(rhs.y),
                    z: self.z.$fname(rhs.z),
                }
            }
        }
        impl<T> $op for &Vec3<T>
            where T: $out_type + Copy,
        {
            type Output = Vec3<T>;
            fn $fname(self, rhs: Self) -> Self::Output {
                Vec3 {
                    x: self.x.$fname(rhs.x),
                    y: self.y.$fname(rhs.y),
                    z: self.z.$fname(rhs.z),
                }
            }
        }
        impl<T> $op_generic for Vec3<T>
            where
                T: $out_type + Copy
                {
                    type Output = Vec3<T>;

                    fn $fname(self, rhs: T) -> Self::Output {
                        Vec3 {
                            x: self.x.$fname(rhs),
                            y: self.y.$fname(rhs),
                            z: self.z.$fname(rhs),
                        }
                    }
                }
        impl<T> $op_generic for &Vec3<T>
            where
                T: $out_type + Copy
                {
                    type Output = Vec3<T>;

                    fn $fname(self, rhs: T) -> Self::Output {
                        Vec3 {
                            x: self.x.$fname(rhs),
                            y: self.y.$fname(rhs),
                            z: self.z.$fname(rhs),
                        }
                    }
                }
    };
}

macro_rules! impl_op_for_type {
    ($op:path, $fname:ident, $type:ty) => {
        impl $op for $type
        {
            type Output = Vec3<f32>;

            fn $fname(self, rhs: Vec3<f32>) -> Self::Output {
                rhs.$fname(self as f32)
            }
        }

    };
}

// Minus
impl_op_for_vec!(std::ops::Sub, std::ops::Sub<Output = T>, std::ops::Sub<T>, sub);

impl ops::SubAssign for Vec3{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for &Vec3{
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Plus
impl_op_for_vec!(std::ops::Add, std::ops::Add<Output = T>, std::ops::Add<T>, add);

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Multiplication & Division
impl_op_for_vec!(std::ops::Mul, std::ops::Mul<Output = T>, std::ops::Mul<T>, mul);
impl_op_for_type!(std::ops::Mul<Vec3>, mul, f32);
impl_op_for_type!(std::ops::Mul<Vec3>, mul, u32);
impl_op_for_type!(std::ops::Mul<Vec3>, mul, usize);

impl ops::Mul<&Vec3<f32>> for f32
{
    type Output = Vec3<f32>;

    fn mul(self, rhs: &Vec3<f32>) -> Self::Output {
        rhs * self
    }
}
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl_op_for_vec!(std::ops::Div, std::ops::Div<Output = T>, std::ops::Div<T>, div);

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl ops::DivAssign<u32> for Vec3
{
    fn div_assign(&mut self, rhs: u32) {
            self.x /= rhs as f32;
            self.y /= rhs as f32;
            self.z /= rhs as f32;
    }
}

impl<T> ops::Index<usize> for Vec3<T>{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds!")
        }
    }
}

// Others
impl Vec3 {
    pub fn length(&self) -> f32{
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3{
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }

    pub fn norm(&self) -> Self {
        let length = self.length();
        self / length
    }

    pub fn reflect(self, n: &Vec3) -> Self {
        self - 2.0*self.dot(n)*n
    }
    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self+cos_theta*n);
        let r_out_parallel = -((1.0-r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn rnd() -> Vec3 {
        let a = rand::random();
        let b = rand::random();
        let c = rand::random();
        Vec3::new(a, b, c)
    }

    pub fn rnd_range(range: Range<f32>) -> Vec3 {
        let a = rand::thread_rng().gen_range(range.clone());
        let b = rand::thread_rng().gen_range(range.clone());
        let c = rand::thread_rng().gen_range(range.clone());
        Vec3::new(a, b, c)
    }
    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::rnd_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().norm()
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}


pub fn rnd_f32() -> f32 {
    rand::random()
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

