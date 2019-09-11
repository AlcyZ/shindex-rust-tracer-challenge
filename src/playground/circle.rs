use crate::canvas::Canvas;
use crate::tuple::Tuple;
use crate::color::Color;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::intersection::intersect;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use crate::playground::utility::save_ppm;
use std::thread::Thread;
use std::sync::mpsc;

pub fn run() {
    let now = Instant::now();

    // setup world stuff
    let canvas_size = 500;
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    // hit color and sphere setup
    let color = Color::new(0.0, 1.0, 0.0);
    let shape = Sphere::new();

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

    save_ppm(canvas);

    println!("Rendering time: {:?}\nSize: {}px", now.elapsed(), canvas_size);
}