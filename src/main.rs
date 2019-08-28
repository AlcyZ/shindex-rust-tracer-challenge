use crate::playground::clock::run as clock_run;
use crate::playground::projectile::run as projectile_run;

mod tuple;
mod color;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod sphere;
mod util;
mod playground;

fn main() {
    clock_run(600);
    projectile_run()
}
