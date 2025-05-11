use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{Material, Scattered};

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scattered {
        let reflected = Vec3::reflect(r_in.dir().clone(), rec.normal);
        let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        let ray = Ray::new(rec.p, reflected);
        let is_scattered = Vec3::dot(ray.dir(), &rec.normal) > 0.0;
        Scattered {
            is_scattered,
            ray,
            attenuation: self.albedo,
        }
    }
}
