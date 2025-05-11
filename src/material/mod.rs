use dielectric::Dielectric;
use lambertian::Lambertian;
use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct Scattered {
    pub is_scattered: bool,
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scattered;
}

#[derive(Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialType {
    pub fn lambertian(albedo: Color) -> Self {
        MaterialType::Lambertian(Lambertian::new(albedo))
    }
    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        MaterialType::Metal(Metal::new(albedo, fuzz))
    }
    pub fn dielectric(refraction_index: f64) -> Self {
        MaterialType::Dielectric(Dielectric::new(refraction_index))
    }
}

impl Material for MaterialType {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scattered {
        match &self {
            MaterialType::Metal(m) => m.scatter(r_in, rec),
            MaterialType::Lambertian(l) => l.scatter(r_in, rec),
            MaterialType::Dielectric(d) => d.scatter(r_in, rec),
        }
    }
}
