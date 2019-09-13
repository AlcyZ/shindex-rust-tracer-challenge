use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread::Thread;
use std::time::Instant;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::intersection::intersect;
use crate::matrix::{Matrix4x4, mul};
use crate::playground::utility::save_ppm;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformation::{rotation_z, scaling, shearing};
use crate::tuple::Tuple;

pub fn run() {
    let now = Instant::now();
    let canvas_size = 200;

    let shrink_y = scaling(1.0, 0.5, 1.0);
    let shrink_x = scaling(0.5, 1.0, 1.0);
    let shrink_x_and_rotate = mul(rotation_z(PI / 2.0), scaling(0.5, 1.0, 1.0));
    let shrink_x_and_skew = mul(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0), scaling(0.5, 1.0, 1.0));

    let handle_a = std::thread::spawn(move || process(canvas_size, None, "default"));
    let handle_b = std::thread::spawn(move || process(canvas_size, Some(shrink_y), "shrink_y"));
    let handle_c = std::thread::spawn(move || process(canvas_size, Some(shrink_x), "shrink_x"));
    let handle_d = std::thread::spawn(move || process(canvas_size, Some(shrink_x_and_rotate), "shrink_x_and_rotate"));
    let handle_e = std::thread::spawn(move || process(canvas_size, Some(shrink_x_and_skew), "shrink_x_and_skew"));

    handle_a.join().unwrap_or_default();
    handle_b.join().unwrap_or_default();
    handle_c.join().unwrap_or_default();
    handle_d.join().unwrap_or_default();
    handle_e.join().unwrap_or_default();

    println!("Rendering time: {:?}\nSize: {}px", now.elapsed(), canvas_size);
}

fn process(canvas_size: usize, transformation: Option<Matrix4x4>, name: &str) {
    // setup world stuff
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;
    // hit color and sphere setup
    let color = Color::new(0.0, 1.0, 0.0);
    let mut shape = Sphere::new();

    if let Some(t) = transformation {
        shape.transform(t);
    }

    let (tx, rx) = mpsc::channel();

    // render
    for y in 0..canvas_size {
        // compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;

        let tx1 = mpsc::Sender::clone(&tx);
        let sphere_clone = shape.clone();

        std::thread::spawn(move || {
            for x in 0..canvas_size {
                // compute the world x coordinate (left = -half, right = half
                let world_x = -half + pixel_size * x as f64;

                // describe the point on the wall that the ray will target
                let position = Tuple::point(world_x, world_y, wall_z);

                let r = Ray::new(ray_origin, (position - ray_origin).normalize()).unwrap();

                if let Some(xs) = intersect(&sphere_clone, &r) {
                    tx1.send(Some((x, y))).unwrap();
                }
            }
        });
    }
    std::thread::spawn(move || tx.send(None).unwrap());

    for rec in rx {
        if let Some(pixels) = rec {
            let (x, y) = pixels;

            canvas.write_pixel(x, y, color);
        }
    }

    let img_name = format!("circle_{}", name);
    save_ppm(canvas, &img_name);
}
