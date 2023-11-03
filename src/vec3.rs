use nalgebra::Vector3;

pub type Vec3 = Vector3<f32>;
pub type Point3 = Vec3;

/*
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
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
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
*/

pub fn reflect(vec: Vec3, n: &Vec3) -> Vec3 {
    vec - 2.0*vec.dot(n)*n
}
pub fn refract(vec: Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-vec).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (vec+cos_theta*n);
    let r_out_parallel = -((1.0-r_out_perp.norm_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

