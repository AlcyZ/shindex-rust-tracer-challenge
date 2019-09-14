use std::f64::consts::PI;
use std::time::Instant;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::mul_by_tuple;
use crate::playground::utility::save_ppm;
use crate::transformation::rotation_y;
use crate::tuple::Tuple;

pub fn run() {
    let now = Instant::now();
    let canvas_size = 2048;

    let color = Color::new(0.0, 1.0, 0.0);
    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let clock_twelve = Tuple::point(0.0, 0.0, 1.0);

    let clock_radius = canvas_size as f64 * (3.0 / 8.0);

    for i in 1..13 {
        let rotation = rotation_y(i as f64 * PI / 6.0);
        let position = mul_by_tuple(rotation, clock_twelve);

        let (_x, _, _z) = position.xyz();

        let x = _x * clock_radius + canvas_size as f64 / 2.0;
        let z = _z * clock_radius + canvas_size as f64 / 2.0;

        canvas.write_pixel(x as usize, z as usize, color);
        canvas.write_pixel(x as usize + 1, z as usize, color);
        canvas.write_pixel(x as usize + 1, z as usize + 1, color);
        canvas.write_pixel(x as usize, z as usize + 1, color);
    }

    save_ppm(canvas, "clock");
    println!("Clock -> Rendering time: {:?}\nSize: {}px", now.elapsed(), canvas_size);
}
