use std::io::{self, Write};

use log::info;

use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    utils::{random_f64, INFINITY},
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_height: i32,
    image_width: i32,
    // aspect_ratio: f64,
    camera_center: Point3,
    pixel_00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixels_samples_scale: f64,
    recursion_depth: usize,
    // vfov: f64,
    // look_from: Vec3,
    // look_at: Vec3,
    // vup: Vec3,
    // u: Vec3,
    // v: Vec3,
    // w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f64
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        recursion_depth: usize,
        vfov: f64,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        focus_dist: f64,
        defocus_angle: f64
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let pixels_samples_scale = 1.0 / (samples_per_pixel as f64);

        let camera_center = look_from;

        // Camera coordinates
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        // Viewport vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Delta between pixels
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left = camera_center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * f64::tan(f64::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_height,
            image_width,
            // aspect_ratio,
            camera_center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixels_samples_scale,
            recursion_depth,
            // vfov,
            // look_from,
            // look_at,
            // vup,
            // u,
            // w,
            // v,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle
        }
    }

    fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: usize) -> Color {
        // Limiting ray bounces
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        };

        let mut rec = HitRecord::default();
        // 0.001 prevents shadow acne
        let interval = Interval::new(0.001, INFINITY);
        let rec = world.hit(r, &interval, &mut rec);
        if rec.is_hit {
            let scatter = rec.mat.scatter(r, rec);
            if scatter.is_scattered {
                return scatter.attenuation * Camera::ray_color(&scatter.ray, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = Color::unit_vector(r.dir());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Point3::random_in_unit_disk();
        self.camera_center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel_00
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 { self.camera_center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let mut stdout = io::stdout();
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        stdout.write_all(header.as_bytes()).unwrap();

        for j in 0..self.image_height {
            info!("Scan lines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_colour += Camera::ray_color(&ray, world, self.recursion_depth);
                }
                write_color(&mut stdout, &(self.pixels_samples_scale * pixel_colour));
            }
        }
        info!("Done!");
    }
}
