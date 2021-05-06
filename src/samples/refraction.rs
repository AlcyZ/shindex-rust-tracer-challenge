use crate::math::transformation::{rotation_x, scaling, translation};
use crate::math::tuple::Tuple;
use crate::pattern::checker::CheckerPattern;
use crate::primitives::plane::Plane;
use crate::primitives::shape::Shape;
use crate::primitives::sphere::Sphere;
use crate::samples::utility::{date_ymd_his, save, SceneCamera};
use crate::scene::color::Color;
use crate::scene::light::PointLight;
use crate::scene::world::World;

use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

pub fn _run() {
    // WORLD SETUP
    let mut world = World::new();
    world.light = Some(PointLight::new(Tuple::point(-5., 5., -5.), Color::white()));

    // SCENE POPULATION
    let mut floor = Plane::new();
    floor.mut_props().set_transform(translation(0., -1., 0.));
    floor.mut_props().set_material_reflective(0.05);
    floor.mut_props().set_pattern(Box::new(CheckerPattern::new(
        Color::new(0.3, 0.3, 0.3),
        Color::new(0.7, 0.7, 0.7),
    )));
    world.objects.push(Box::new(floor));

    let mut back_wall = Plane::new();
    back_wall
        .mut_props()
        .set_transform(translation(0., 0., 40.) * scaling(1., 1., 1.) * rotation_x(PI / 2.));
    back_wall
        .mut_props()
        .set_material_color(Color::new(0.2, 0.2, 0.2));
    world.objects.push(Box::new(back_wall));

    let mut center_sphere = Sphere::glass();
    center_sphere
        .mut_props()
        .set_material_color(Color::new(0., 0.3, 0.));
    center_sphere.mut_props().set_material_ambient(0.1);
    center_sphere.mut_props().set_material_diffuse(0.1);
    center_sphere.mut_props().set_material_reflective(0.9);
    center_sphere.mut_props().set_material_specular(1.);
    center_sphere.mut_props()._set_material_shininess(300.);
    world.objects.push(Box::new(center_sphere));

    let mut left_behind_sphere = Sphere::new();
    left_behind_sphere
        .mut_props()
        .set_transform(translation(-2., -0.25, 2.) * scaling(0.75, 0.75, 0.75));
    left_behind_sphere
        .mut_props()
        .set_material_color(Color::new(1., 0., 0.));
    world.objects.push(Box::new(left_behind_sphere));

    let mut right_behind_sphere = Sphere::new();
    right_behind_sphere
        .mut_props()
        .set_transform(translation(2., -0.25, 2.) * scaling(0.75, 0.75, 0.75));
    right_behind_sphere
        .mut_props()
        .set_material_color(Color::new(0., 0., 1.));
    world.objects.push(Box::new(right_behind_sphere));

    let mut left_before_sphere = Sphere::glass();
    left_before_sphere
        .mut_props()
        .set_transform(translation(-1.3, 0.25, -2.) * scaling(1.25, 1.25, 1.25));
    left_before_sphere
        .mut_props()
        .set_material_color(Color::new(0., 0., 0.3));
    left_before_sphere.mut_props().set_material_ambient(0.1);
    left_before_sphere.mut_props().set_material_diffuse(0.1);
    left_before_sphere.mut_props().set_material_reflective(0.9);
    left_before_sphere.mut_props().set_material_specular(1.);
    left_before_sphere.mut_props()._set_material_shininess(300.);
    world.objects.push(Box::new(left_before_sphere));

    let mut right_before_sphere = Sphere::glass();
    right_before_sphere
        .mut_props()
        .set_transform(translation(1.3, 0.25, -2.) * scaling(1.25, 1.25, 1.25));
    right_before_sphere
        .mut_props()
        .set_material_color(Color::new(0.3, 0., 0.));
    right_before_sphere.mut_props().set_material_ambient(0.1);
    right_before_sphere.mut_props().set_material_diffuse(0.1);
    right_before_sphere.mut_props().set_material_reflective(0.9);
    right_before_sphere.mut_props().set_material_specular(1.);
    right_before_sphere
        .mut_props()
        ._set_material_shininess(300.);
    world.objects.push(Box::new(right_before_sphere));

    // CAMERA SETUP
    let camera = SceneCamera::new(Tuple::point(0., 2.0, -8.), Tuple::point(0., 0., 0.));
    let date = date_ymd_his();
    let w = Arc::new(world);

    let start = Instant::now();
    println!("start rendering");

    let canvas = camera.get_camera().render_multi_threaded(w);
    save("refraction", canvas, 1, &date);

    println!(
        "finished rendering after {:02} seconds",
        start.elapsed().as_secs_f64()
    );
}
