mod color;
mod ray;
mod vec3;

use core::f64;
use std::{fs::{self, File}, io::{self, Write}, path::Path};
use color::{write_color, Color};
use env_logger::Builder;
use log::{info, LevelFilter};
use ray::Ray;
use vec3::{Point3, Vec3};

const DEBUG_FILE: &'static str = "debug.log";

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    // let a = Point3::dot(r.dir(), r.dir());
    // let b = -2.0 * Point3::dot(r.dir(), &oc);
    // let c = Point3::dot(&oc, &oc) - radius * radius;
    // let discriminant = b * b - 4.0 * a * c;
    let a = r.dir().length_squared();
    let h = Point3::dot(r.dir(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t =  hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    }
    let unit_direction = Color::unit_vector(r.dir());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {

    let path = Path::new(DEBUG_FILE);
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    let file = File::create(path).unwrap();
    Builder::new()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .filter_level(LevelFilter::Debug)
        .init();

    let mut stdout = io::stdout();

    // Image height/width ratios
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Delta between pixels
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel_double0 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    stdout.write_all(header.as_bytes()).unwrap();

    for j in 0..image_height {
        info!("Scan lines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel_double0 + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut stdout, &pixel_color);
        }
    }
    info!("Done!");
}
