use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use crate::math::transformation::{
    rotation_x, rotation_y, rotation_z, translation, view_transform,
};
use crate::pattern::checker::CheckerPattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

pub mod sequence;
pub mod simple_sphere_world;
mod test_one;

pub fn run() {
    let width = 200;
    let height = 100;

    println!("render with resolution {}x{}", width, height);
    let time = Instant::now();

    // add and position scene objects
    let mut plane = Plane::new();
    plane.mut_props().set_transform(translation(0., -1., 0.));
    // plane.mut_props().set_material_reflective(1.0);
    // plane.mut_props().set_material_diffuse(0.0);

    let mut left_wall = Plane::new();
    left_wall
        .mut_props()
        .set_pattern(Box::new(CheckerPattern::new(
            Color::new(0.75, 0.75, 0.75),
            Color::new(0.0, 0.0, 0.0),
        )));
    left_wall.mut_props().set_material_reflective(1.0);
    left_wall.mut_props().set_material_diffuse(1.0);
    let left_wall_transform = translation(0., 0., 3.) * rotation_x(PI / 2.);
    left_wall.mut_props().set_transform(left_wall_transform);

    let mut test = Sphere::new();
    // test.mut_props().set_material_reflective(1.0);
    // test.mut_props().set_material_diffuse(0.0);

    // populate world
    let mut world = World::new();
    world.light = Some(PointLight::new(
        Tuple::point(-3., 3., -5.),
        Color::new(1., 1., 1.),
    ));

    world.objects.push(Box::new(plane));
    world.objects.push(Box::new(left_wall));
    world.objects.push(Box::new(test));

    let w_ref = Arc::new(world);

    for i in 0..60 {
        let foo = -3. + (i as f64 / 10.);

        // camera setup
        let mut camera = Camera::new(width, height, PI / 3.);
        camera.transform = view_transform(
            Tuple::point(foo, 2., -5.),
            // Tuple::point(0., 2., -5.),
            Tuple::point(0., 0., 0.),
            Tuple::direction(0., 1., 0.),
        );

        let w_ref = w_ref.clone();

        // // rendering
        let canvas = camera.render_multi_threaded(w_ref);
        let ppm = canvas.to_ppm();
        std::fs::write(&format!("./dist/{}_vid.ppm", i), ppm).expect("could not save image");

        let duration = time.elapsed().as_secs_f64();
        println!("render finished after {:.4} secs", duration);
    }

    // camera setup
    // let mut camera = Camera::new(width, height, PI / 3.);
    // camera.transform = view_transform(
    //     Tuple::point(3., 2., -5.),
    //     // Tuple::point(0., 2., -5.),
    //     Tuple::point(0., 0., 0.),
    //     Tuple::direction(0., 1., 0.),
    // );
    //
    // // rendering
    // let canvas = camera.render_multi_threaded(Arc::new(world));
    // let ppm = canvas.to_ppm();
    // std::fs::write("./dist/test.ppm", ppm).expect("could not save image");
    //
    // let duration = time.elapsed().as_secs_f64();
    // println!("render finished after {:.4} secs", duration);
}
