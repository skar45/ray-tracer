mod vec3;
mod color;

use std::io::{self, Write};

fn write_stdout(buf: String) {
    io::stdout().write_all(buf.as_bytes()).unwrap();
}

fn main() {
    let image_height = 256;
    let image_width = 256;

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    write_stdout(header);

    for i in 0..image_height {
        for j in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write_stdout(format!("{} {} {}\n", ir, ig, ib));
        }
    }
}
