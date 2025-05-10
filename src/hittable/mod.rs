use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub is_hit: bool,
}

impl HitRecord {
    pub fn self_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let front_face = Vec3::dot(r.dir(), &outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self.front_face = front_face;
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Point3::new(0.0, 0.0, 0.0);
        HitRecord {
            p,
            normal,
            t: 0.0,
            front_face: false,
            is_hit: false,
        }
    }
}

pub trait Hittable {
    fn hit<'a>(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> &'a mut HitRecord;
}
