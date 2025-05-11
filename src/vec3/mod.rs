use std::{
    fmt::{Display, Formatter},
    ops,
};

use crate::utils::{random_f64, random_min_max};

#[derive(Clone, Copy)]
pub struct Vec3(f64, f64, f64);

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3(e0, e1, e2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        let e = self;
        e.0 * e.0 + e.1 * e.1 + e.2 * e.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.0) < s) && (f64::abs(self.1) < s) && (f64::abs(self.2) < s)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_min_max(-1.0, 1.0);
            let lensq = p.length_squared();
            // Deal with underflow when squaring small coordinates
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit = Vec3::random_unit_vector();
        // Check if in the same hemisphere as the normal
        if Vec3::dot(&on_unit, normal) > 0.0 {
            on_unit
        } else {
            -on_unit
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(&v, &n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_on_etat: f64) -> Vec3 {
        let cost_theta = f64::min(Vec3::dot(&(-uv), &n), 1.0);
        let r_out_perp = etai_on_etat * (uv + cost_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3(random_f64(), random_f64(), random_f64())
    }

    pub fn random_min_max(min: f64, max: f64) -> Vec3 {
        Vec3(
            random_min_max(min, max),
            random_min_max(min, max),
            random_min_max(min, max),
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs;
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

#[test]
fn smoke() {
    let point = Point3::new(0.0, 0.0, 0.0);
    let a = 2.0 * point;
    let a = a * 2.0;
    let a = a / 1.0;
    println!("point {}", a.1);
}
