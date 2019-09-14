use crate::playground::circle::run as circle_run;
use crate::playground::clock::run as clock_run;
use crate::playground::projectile::run as projectile_run;
use crate::playground::sphere::run as sphere_run;

mod tuple;
mod color;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod light;
mod material;
mod sphere;
mod intersection;
mod util;
mod playground;

fn main() {
//    clock_run(600);
//    projectile_run()
//    circle_run();

    sphere_run();
}
