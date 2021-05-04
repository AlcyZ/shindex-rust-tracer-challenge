use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use crate::math::transformation::{translation, view_transform};
use crate::pattern::checker::CheckerPattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use chrono::{Datelike, Local, Timelike, Utc};
use std::f64::consts::PI;
use std::sync::Arc;

// const SURFACE_WIDTH: usize = 640;
// const SURFACE_HEIGHT: usize = 480;
const SURFACE_WIDTH: usize = 320;
const SURFACE_HEIGHT: usize = 240;

pub fn run() {
    let mut world = World::new();
    world.light = Some(PointLight::new(
        Tuple::point(-10., 10., -10.),
        Color::white(),
    ));

    let mut floor = Plane::new();
    floor.mut_props().set_transform(translation(0., -1., 0.));
    floor.mut_props().set_pattern(Box::new(CheckerPattern::new(
        Color::new(0.1, 0.1, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));
    floor.mut_props().set_material_reflective(0.1);
    world.objects.push(Box::new(floor));

    let mut center_sphere = Sphere::new();
    center_sphere.mut_props().set_material_reflective(0.1);
    world.objects.push(Box::new(center_sphere));

    // render the world
    let mut camera = Camera::new(SURFACE_WIDTH, SURFACE_HEIGHT, PI / 3.);
    camera.transform = view_transform(
        Tuple::point(0., 2., -5.),
        Tuple::point(0., 0., 0.),
        Tuple::direction(0., 1., 0.),
    );

    let canvas = camera.render(Arc::new(world));
    let ppm_image = canvas.to_ppm();

    let utc = Local::now();
    let destination = format!(
        "./dist/{year}-{month:02}-{day:02}_{hour:02}-{min:02}-{sec:02}-simple_where_world.ppm",
        year = utc.year(),
        month = utc.month(),
        day = utc.day(),
        hour = utc.hour(),
        min = utc.minute(),
        sec = utc.second(),
    );

    std::fs::write(destination, ppm_image).unwrap();

    println!("finished rendering");
}
