use std::fs::File;
use std::io::Write;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::{normalize, point, Tuple, vector, tuple_add, tuple_mul_scalar};
use std::time::Instant;

mod tuple;
mod color;
mod canvas;
mod matrix;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, pro: Projectile, canvas: &mut Canvas) -> Projectile {
    let old_vel = pro.velocity;

    let position = tuple_add(pro.position, old_vel);
    let velocity = tuple_add(tuple_add(old_vel, env.gravity), env.wind);

    let color = Color::new(0__f64, 1_f64, 0_f64);
    let w = pro.position[0] as usize;
    let h = canvas.height - pro.position[1] as usize;

    canvas.write_pixel(w, h, color);

    Projectile { position, velocity }
}

fn main() {
    let now = Instant::now();

    let start = point(0_f64, 1_f64, 0_f64);

    let mut velocity = vector(1_f64, 1.8, 0_f64);
    normalize(&mut velocity);
    velocity = tuple_mul_scalar(velocity, 11.25);

    let mut projectile = Projectile { position: start, velocity };

    let gravity = vector(0_f64, -0.1, 0_f64);
    let wind = vector(-0.01, 0_f64, 0_f64);
    let env = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);


    while projectile.position[1] > 0 as f64 {
        projectile = tick(&env, projectile, &mut c);
    }

    let ppm = c.to_ppm();
    let mut f = File::create("./result.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");

    println!("{:?}", now.elapsed());
}
