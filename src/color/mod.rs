use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color<T: Write>(stdout: &mut T, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    stdout
        .write_all(format!("{} {} {}\n", rbyte, gbyte, bbyte).as_bytes())
        .unwrap();
}
