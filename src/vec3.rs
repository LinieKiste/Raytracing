use core::fmt;
use std::ops;

use rand::{rngs::ThreadRng, Rng};

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
}

// Minus
impl<T> ops::Sub for Vec3<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T> ops::Sub for &Vec3<T>
where
    T: ops::Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T> ops::Sub<T> for Vec3<T>
where
    T: ops::Sub<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

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

// Plus
impl<T: ops::Add<Output = T>> ops::Add for Vec3<T>{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T> ops::Add<T> for Vec3<T>
where
    T: ops::Add<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Multiplication & Division
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl<T> ops::Mul<T> for Vec3<T> 
where
    T: ops::Mul<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Mul<Vec3<f32>> for f32
{
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Self::Output {
        rhs * self
    }
}
impl ops::Mul<Vec3<f32>> for u32
{
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Self::Output {
        rhs * self as f32
    }
}
impl ops::Mul<Vec3<f32>> for usize
{
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Self::Output {
        rhs * self as f32
    }
}
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> ops::Div for Vec3<T>
where
    T: ops::Div<Output = T> + Copy
{
    type Output = Vec3<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }

}
impl<T> ops::Div<T> for &Vec3<T>
where
    T: ops::Div<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl<T> ops::Div<T> for Vec3<T>
where
    T: ops::Div<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl<T: VecConvert> ops::DivAssign<T> for Vec3
{
    fn div_assign(&mut self, rhs: T) {
            self.x /= rhs.to_f32();
            self.y /= rhs.to_f32();
            self.z /= rhs.to_f32();
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

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::rnd_range(-1., 1.);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }
    fn random_unit_vector() -> Vec3 {
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

    pub fn rnd() -> Vec3 {
        let a = rand::random();
        let b = rand::random();
        let c = rand::random();
        Vec3::new(a, b, c)
    }

    pub fn rnd_range(min: f32, max: f32) -> Vec3 {
        let a = rand::thread_rng().gen_range(min..max);
        let b = rand::thread_rng().gen_range(min..max);
        let c = rand::thread_rng().gen_range(min..max);
        Vec3::new(a, b, c)
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// loose conversions for vectors
trait VecConvert {
    fn to_f32(&self) -> f32;
}
impl VecConvert for u32 {
    fn to_f32(&self) -> f32 { *self as f32 }
}

