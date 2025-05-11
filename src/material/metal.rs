use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::{Material, Scattered};

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scattered {
        let reflected = Vec3::reflect(r_in.dir().clone(), rec.normal);
        Scattered {
            is_scattered: true,
            attenuation: self.albedo,
            ray: Ray::new(rec.p, reflected),
        }
    }
}
