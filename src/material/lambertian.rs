use super::{Material, Scattered};
use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> super::Scattered {
        let mut scatter_directon = rec.normal + Vec3::random_unit_vector();
        if scatter_directon.near_zero() {
            scatter_directon = rec.normal;
        }
        Scattered {
            is_scattered: true,
            attenuation: self.albedo.clone(),
            ray: Ray::new(rec.p, scatter_directon),
        }
    }
}
