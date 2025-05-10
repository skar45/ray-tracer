use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList<T: Hittable> {
    objects: Vec<Box<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit<'a>(
        &self,
        r: &crate::ray::Ray,
        ray_t: &Interval,
        rec: &'a mut HitRecord,
    ) -> &'a mut HitRecord {
        let mut temp_rec = HitRecord::default();
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let new_rec = obj.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec);
            if new_rec.is_hit {
                closest_so_far = new_rec.t;
                *rec = new_rec.clone();
            }
        }

        rec
    }
}
