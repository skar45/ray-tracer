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

    let material_ground = MaterialType::lambertian(Color::new(0.6, 0.6, 0.0));
    world.add_obj(Sphere::new(
        Point3::new(0.0, -1000.0, 1000.0),
        1000.0,
        material_ground,
    ));

    let material1 = MaterialType::dielectric(1.5);
    world.add_obj(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = MaterialType::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add_obj(Sphere::new(
        Point3::new(-4.0, 1.0, -1.0),
        1.0,
        material2,
    ));

    let material3 = MaterialType::metal(Color::new(0.7, 0.6, 0.5), 0.3);
    world.add_obj(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    let material4 = MaterialType::dielectric(1.5);
    world.add_obj(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.6,
        material4,
    ));

    let material5 = MaterialType::metal(Color::new(0.1, 0.2, 0.9), 0.5);
    world.add_obj(Sphere::new(
        Point3::new(-4.0, -1.0, 2.0),
        0.5,
        material5,
    ));

    let material6 = MaterialType::lambertian(Color::random());
    world.add_obj(Sphere::new(
        Point3::new(4.0, 4.0, -6.0),
        0.5,
        material6,
    ));

    let material7 = MaterialType::lambertian(Color::random());
    world.add_obj(Sphere::new(
        Point3::new(1.0, -2.0, -2.0),
        0.4,
        material7,
    ));

    let material8 = MaterialType::dielectric(0.5);
    world.add_obj(Sphere::new(
        Point3::new(6.0, 7.0, 8.0),
        0.3,
        material8,
    ));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let recursion_depth = 50;

    let vfov = 20.0;
    let look_from = Point3::new(1.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let focus_dist = 10.0;
    let defocus_angle = 0.6;

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
