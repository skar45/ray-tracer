use std::io::{self, Write};

use log::info;

use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::INFINITY,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_height: i32,
    image_width: i32,
    aspect_ratio: f64,
    camera_center: Point3,
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
        let center = Point3::new(0.0, 0.0, 0.0);

        // Viewport vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Delta between pixels
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_height,
            image_width,
            aspect_ratio,
            camera_center: center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
        let mut rec = HitRecord::default();
        let interval = Interval::new(0.0, INFINITY);
        let rec = world.hit(r, &interval, &mut rec);
        if rec.is_hit {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = Color::unit_vector(r.dir());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let mut stdout = io::stdout();
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        stdout.write_all(header.as_bytes()).unwrap();

        for j in 0..self.image_height {
            info!("Scan lines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel_00
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);
                let pixel_color = Camera::ray_color(&ray, world);
                write_color(&mut stdout, &pixel_color);
            }
        }
        info!("Done!");
    }
}
