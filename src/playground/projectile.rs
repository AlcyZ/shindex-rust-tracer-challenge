use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::tuple::*;

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

    let position = pro.position + old_vel;
    let velocity = old_vel + env.gravity + env.wind;
    let (x, y, _) = pro.position.xyz();

    let color = Color::new(0__f64, 1_f64, 0_f64);
    let w = x as usize;
    let h = canvas.height - (y as usize);

    canvas.write_pixel(w, h, color);

    Projectile { position, velocity }
}

pub fn run() {
    let now = Instant::now();

    let start = Tuple::point(0_f64, 1_f64, 0_f64);

    let mut velocity = Tuple::vector(1_f64, 1.8, 0_f64);
    velocity = velocity.normalize() * 11.5;

    let mut projectile = Projectile { position: start, velocity };

    let gravity = Tuple::vector(0_f64, -0.1, 0_f64);
    let wind = Tuple::vector(-0.01, 0_f64, 0_f64);
    let env = Environment { gravity, wind };

    let mut c = Canvas::new(900, 550);

    let mut y = 1.0;
    while y > 0 as f64 {
        projectile = tick(&env, projectile, &mut c);
        let (_, _y, _) = projectile.position.xyz();
        y = _y;
    }

    let ppm = c.to_ppm();
    let mut f = File::create("./PuttingItTogether/projectile.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");

    println!("{:?}", now.elapsed());
}
