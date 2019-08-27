use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::mul_by_tuple;
use crate::transformation::rotation_y;
use crate::tuple::point;

pub fn run(size: usize) {
    let origin = point(0.0, 0.0, 0.0);
    let color = Color::new(0.0, 1.0, 0.0);
    let mut canvas = Canvas::new(size, size);

    let clock_twelve = point(0.0, 0.0, 1.0);

    let rotation = rotation_y(PI / 6.0);
    let clock_radius = size as f64 * (3.0 / 8.0);

    println!("{}", clock_radius);

    let mut next = clock_twelve;
    for i in 1..13 {
        let rotation = rotation_y(i as f64 * PI / 6.0);
        let position = mul_by_tuple(rotation, clock_twelve);

        let x = position[0] * clock_radius + size as f64 / 2.0;
        let z = position[2] * clock_radius + size as f64 / 2.0;

        canvas.write_pixel(x as usize, z as usize, color);
        canvas.write_pixel(x as usize + 1, z as usize, color);
        canvas.write_pixel(x as usize + 1, z as usize + 1, color);
        canvas.write_pixel(x as usize, z as usize + 1, color);
    }

    let ppm = canvas.to_ppm();
    let mut f = File::create("./PuttingItTogether/clock.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
