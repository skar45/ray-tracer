use crate::{
    hittable::{HitRecord, Hittable}, interval::Interval, material::MaterialType, ray::Ray, vec3::Point3
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: MaterialType
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: MaterialType) -> Self {
        Sphere {
            center,
            mat,
            radius: f64::max(0.0, radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> &'a mut HitRecord {
        let oc = self.center - *r.origin();
        let a = r.dir().length_squared();
        let h = Point3::dot(r.dir(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            rec.is_hit = false;
            return rec;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                rec.is_hit = false;
                return rec;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.self_face_normal(r, outward_normal);
        rec.is_hit = true;
        rec.mat = self.mat.clone();

        return rec;
    }
}
