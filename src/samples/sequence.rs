use crate::color::Color;
use crate::light::PointLight;
use crate::math::matrix::M4;
use crate::math::transformation::{rotation_x, rotation_y, rotation_z, scaling, translation};
use crate::pattern::checker::CheckerPattern;
use crate::pattern::gradient::GradientPattern;

use crate::pattern::Pattern;
use crate::plane::Plane;
use crate::samples::utility::{date_ymd_his, save, SceneCamera};
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;

use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

pub fn _run() {
    let mut world = World::new();
    world.light = Some(PointLight::new(Tuple::point(0., 5., -10.), Color::white()));

    let mut center_sphere = Sphere::new();
    center_sphere.mut_props().set_material_diffuse(0.);
    center_sphere.mut_props().set_material_reflective(1.);
    world.objects.push(Box::new(center_sphere));

    let mut left_sphere = Sphere::new();
    left_sphere
        .mut_props()
        .set_material_color(Color::new(0.9, 0.6, 0.2));
    left_sphere.mut_props().set_material_diffuse(0.4);
    left_sphere.mut_props().set_material_specular(0.2);
    left_sphere.mut_props()._set_material_shininess(75.);

    let left_sphere_transform =
        translation(-1., -0.5, -1.5) * scaling(0.5, 0.5, 0.5) * rotation_x(0.) * M4::identity();
    left_sphere.mut_props().set_transform(left_sphere_transform);
    world.objects.push(Box::new(left_sphere));

    let mut right_sphere = Sphere::glass();
    // right_sphere
    //     .mut_props()
    //     .set_material_color(Color::new(0.4, 0.5, 0.4));
    right_sphere.mut_props().set_material_reflective(0.9);
    right_sphere.mut_props().set_material_diffuse(0.1);
    right_sphere.mut_props().set_material_ambient(0.1);

    let right_sphere_transform =
        translation(1., -0.25, -1.75) * scaling(0.75, 0.75, 0.75) * rotation_x(0.) * M4::identity();
    right_sphere
        .mut_props()
        .set_transform(right_sphere_transform);
    world.objects.push(Box::new(right_sphere));

    let mut floor = Plane::new();
    floor.mut_props().set_transform(translation(0., -1., 0.));
    floor.mut_props().set_pattern(Box::new(CheckerPattern::new(
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.8, 0.8, 0.8),
    )));
    floor.mut_props().set_material_reflective(0.1);
    world.objects.push(Box::new(floor));

    let mut right_wall = Plane::new();
    // right_wall
    //     .mut_props()
    //     .set_pattern(Box::new(CheckerPattern::new(
    //         Color::new(0.0, 0.5, 0.0),
    //         Color::new(0.8, 0.8, 0.8),
    //     )));
    let mut gradient_pattern =
        GradientPattern::new(Color::new(0.0, 0.5, 0.0), Color::new(0.8, 0.8, 0.8));
    let asd_transform = translation(-1., 0., 0.) * scaling(5., 5., 5.) * M4::identity();
    gradient_pattern.mut_props().set_transform(asd_transform);
    right_wall
        .mut_props()
        .set_pattern(Box::new(gradient_pattern));
    right_wall.mut_props().set_material_reflective(0.05);
    let right_wall_transform =
        translation(5., 0., 0.) * rotation_y(PI * 1.75) * rotation_z(PI / 2.) * M4::identity();
    right_wall.mut_props().set_transform(right_wall_transform);
    world.objects.push(Box::new(right_wall));

    let mut left_wall = Plane::new();
    left_wall
        .mut_props()
        .set_pattern(Box::new(CheckerPattern::new(
            Color::new(0.0, 0.0, 0.5),
            Color::new(0.8, 0.8, 0.8),
        )));
    left_wall.mut_props().set_material_reflective(0.2);
    let left_wall_transform =
        translation(-5., 0., 0.) * rotation_y(PI * 0.25) * rotation_z(PI / 2.) * M4::identity();
    left_wall.mut_props().set_transform(left_wall_transform);
    world.objects.push(Box::new(left_wall));

    let mut camera = SceneCamera::new(Tuple::point(0., 2., -5.), Tuple::point(0., 0., 0.));
    let date = date_ymd_his();
    let w = Arc::new(world);

    let start = Instant::now();
    println!("start rendering");

    for foo in 0..1 {
        camera._move_camera(0., 0.1, 0.);

        let w = w.clone();
        let canvas = camera.get_camera().render_multi_threaded(w);
        save("sequence", canvas, foo + 1, &date);
    }

    println!(
        "finished rendering after {:02} seconds",
        start.elapsed().as_secs_f64()
    );
}
