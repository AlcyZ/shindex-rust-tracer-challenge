use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use crate::math::transformation::{rotation_x, rotation_y, scaling, translation, view_transform};
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use std::f64::consts::PI;
use std::time::Instant;

mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod math;
mod plane;
mod ray;
mod shape;
mod sphere;
mod tuple;
mod world;

fn main() {
    let width = 1980;
    let height = 1080;

    println!("render with resolution {}x{}", width, height);

    let time = Instant::now();

    let mut middle = Sphere::new();
    middle.mut_props().set_transform(translation(-0.5, 1., 0.5));
    middle
        .mut_props()
        .set_material_color(Color::new(0.1, 1., 0.5));
    middle.mut_props().set_material_diffuse(0.7);
    middle.mut_props().set_material_specular(0.3);

    let mut right = Sphere::new();
    right
        .mut_props()
        .set_transform(translation(1.5, 0.5, 0.1) * scaling(0.5, 0.5, 0.5));
    right
        .mut_props()
        .set_material_color(Color::new(0.5, 1., 0.1));
    right.mut_props().set_material_diffuse(0.7);
    right.mut_props().set_material_specular(0.3);

    let mut left = Sphere::new();
    left.mut_props()
        .set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.mut_props()
        .set_material_color(Color::new(1., 0.8, 0.1));
    left.mut_props().set_material_diffuse(0.7);
    left.mut_props().set_material_specular(0.3);

    let mut front = Sphere::new();
    front.mut_props()
        .set_transform(translation(0.5, 0.2, -1.) * scaling(0.2, 0.2, 0.2));
    front.mut_props()
        .set_material_color(Color::new(0.8, 0.7, 0.3));
    front.mut_props().set_material_diffuse(0.7);
    front.mut_props().set_material_specular(0.3);
    front.mut_props()._set_material_shininess(50.0);

    let plane = Plane::new();

    let mut wall = Plane::new();
    wall.mut_props()
        .set_material_color(Color::new(0., 0.8, 0.15));

    wall.mut_props()
        .set_transform(translation(0., 2., 3.) * scaling(2., 2., 2.) * rotation_x(PI / 2.));

    let point_light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let mut world = World::new();
    world.light = Some(point_light);

    world.objects.push(Box::new(plane));
    world.objects.push(Box::new(wall));
    world.objects.push(Box::new(middle));
    world.objects.push(Box::new(front));
    world.objects.push(Box::new(right));
    world.objects.push(Box::new(left));

    let mut camera = Camera::new(width, height, PI / 3.);
    camera.transform = view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::direction(0., 1., 0.),
    );

    let canvas = camera.render(&world);

    let ppm = canvas.to_ppm();
    std::fs::write("./foo.ppm", ppm).unwrap();

    let duration = time.elapsed().as_secs_f64();

    println!("render finished after {:.4} secs", duration);
}
