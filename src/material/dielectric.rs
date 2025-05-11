use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_f64, vec3::Vec3};

use super::{Material, Scattered};

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Dielectric {
    // Schlick's approximation for reflectance
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Scattered {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = Vec3::unit_vector(r_in.dir());
        let cos_theta = f64::min(Vec3::dot(&(-unit_dir), &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_f64() {
            Vec3::reflect(unit_dir, rec.normal)
        } else {
            Vec3::refract(unit_dir, rec.normal, ri)
        };
        Scattered {
            is_scattered: true,
            ray: Ray::new(rec.p, direction),
            attenuation,
        }
    }
}
