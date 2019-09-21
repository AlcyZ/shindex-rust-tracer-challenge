use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

use crate::camera::{Camera, render};
use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::mul;
use crate::playground::utility::{save_png, save_ppm};
use crate::sphere::Sphere;
use crate::transformation::{rotation_x, rotation_y, scaling, translation, view_transform};
use crate::tuple::Tuple;
use crate::world::World;

pub fn run() {
    let now = Instant::now();

    let floor = floor();
    let left_wall = left_wall();
    let right_wall = right_wall();
    let left = left();
    let right = right();
    let middle = middle();
    let light = light();

    let mut world = World::new();
    world.add_object(floor);
    world.add_object(left_wall);
    world.add_object(right_wall);
    world.add_object(left);
    world.add_object(right);
    world.add_object(middle);
    world.change_light(light);

    let width = 4096;
    let height = 2160;
    let camera = camera(width, height);
    let canvas = render(camera, Arc::new(world));

    save_png(canvas, "many-spheres");
    println!("Projectile -> Rendering time: {:?}\nSize: width: {}px, height: {}px", now.elapsed(), width, height);
}

fn camera(width: usize, height: usize) -> Camera {
    let mut camera = Camera::new(width, height, PI / 3.0);
    camera.transform(view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::point(0.0, 1.0, 0.0),
    ));

    camera
}

fn light() -> PointLight {
    PointLight::from_cords(-10.0, 10.0, -10.0, Color::white())
}

fn left() -> Sphere {
    let mut left = Sphere::new();

    left.transform(mul(translation(-1.5, 0.33, -0.75), scaling(0.33, 0.33, 0.33)));
    left.change_color(Color::new(1.0, 0.8, 0.1));
    left.change_specular(0.7);
    left.change_diffuse(0.3);

    left
}

fn right() -> Sphere {
    let mut right = Sphere::new();

    right.transform(mul(translation(1.5, 0.5, -0.5), scaling(0.5, 0.5, 0.5)));
    right.change_color(Color::new(0.5, 1.0, 0.1));
    right.change_specular(0.7);
    right.change_diffuse(0.3);

    right
}

fn middle() -> Sphere {
    let mut middle = Sphere::new();

    middle.transform(translation(-0.5, 1.0, 0.5));
    middle.change_color(Color::new(0.1, 1.0, 0.5));
    middle.change_specular(0.7);
    middle.change_diffuse(0.3);

    middle
}

fn right_wall() -> Sphere {
    let mut right_wall = Sphere::new();

    let transform = mul(translation(0.0, 0.0, 5.0), rotation_y(-PI / 4.0));
    let transform = mul(transform, rotation_x(PI / 2.0));
    let transform = mul(transform, scaling(10.0, 0.01, 10.0));

    right_wall.transform(transform);

    right_wall
}

fn left_wall() -> Sphere {
    let mut left_wall = Sphere::new();

    let transform = mul(translation(0.0, 0.0, 5.0), rotation_y(PI / 4.0));
    let transform = mul(transform, rotation_x(PI / 2.0));
    let transform = mul(transform, scaling(10.0, 0.01, 10.0));

    left_wall.transform(transform);

    left_wall
}

fn floor() -> Sphere {
    let mut floor = Sphere::new();
    floor.transform(scaling(10.0, 0.01, 10.0));
    floor.change_color(Color::new(1.0, 0.9, 0.9));
    floor.change_specular(0.0);

    floor
}
