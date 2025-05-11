mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use env_logger::Builder;
use log::LevelFilter;
use material::MaterialType;
use objects::hittable_list::HittableList;
use objects::sphere::Sphere;
use std::fs::{self, File};
use std::path::Path;
use vec3::{Point3, Vec3};

const DEBUG_FILE: &'static str = "debug.log";

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

    let mut world = HittableList::new();
    let material_ground = MaterialType::lambertian(Color::new(0.8, 0.8, 0.0));
    let material_center = MaterialType::lambertian(Color::new(0.1, 0.2, 0.5));
    let material_left = MaterialType::dielectric(1.50);
    let material_bubble = MaterialType::dielectric(1.0 / 1.50);
    let material_right = MaterialType::metal(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add_obj(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add_obj(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add_obj(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add_obj(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add_obj(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let recursion_depth = 50;

    let vfov = 20.0;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let focus_dist = 3.4;
    let defocus_angle = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        recursion_depth,
        vfov,
        look_from,
        look_at,
        vup,
        focus_dist,
        defocus_angle
    );
    camera.render(&world);
}
