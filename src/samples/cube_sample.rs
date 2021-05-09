use crate::math::transformation::{scaling, translation};
use crate::math::tuple::Tuple;
use crate::pattern::checker::CheckerPattern;
use crate::primitives::cube::Cube;
use crate::primitives::shape::Shape;
use crate::primitives::sphere::Sphere;
use crate::samples::utility::{
    date_ymd_his, glass_material, mirror_material, save, water_material, SceneCamera,
};
use crate::scene::shading::color::Color;
use crate::scene::shading::light::PointLight;
use crate::scene::world::World;
use std::sync::Arc;
use std::time::Instant;

pub fn _run() {
    let mut world = World::new();
    world.light = Some(PointLight::new(Tuple::point(-2., 5., -5.), Color::white()));

    // WORLD POPULATION
    let mut room = Cube::new();
    room.mut_props()
        .set_transform(translation(0., 6.49, 0.) * scaling(7.5, 7.5, 7.5));
    room.mut_props().set_pattern(Box::new(CheckerPattern::new(
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.8, 0.8, 0.8),
    )));
    room.mut_props().set_material_reflective(0.2);
    world.objects.push(Box::new(room));

    let mut cube = Cube::new();
    cube.mut_props().set_material_color(Color::new(0.2, 0., 0.));
    cube.mut_props().set_material_diffuse(0.1);
    cube.mut_props().set_material_ambient(0.1);
    cube.mut_props().set_material_reflective(0.9);
    cube.mut_props().set_material_transparency(1.0);
    cube.mut_props().set_material_refractive_index(1.5);

    world.objects.push(Box::new(cube));

    // SPHERES AROUND CENTER CUBE
    let mut left_sphere = Sphere::new();
    left_sphere
        .mut_props()
        .set_transform(translation(-2.5, -0.51, 0.) * scaling(0.5, 0.5, 0.5));
    water_material(left_sphere.mut_props());
    world.objects.push(Box::new(left_sphere));

    let mut right_sphere = Sphere::new();
    right_sphere
        .mut_props()
        .set_transform(translation(2.5, -0.51, 0.) * scaling(0.5, 0.5, 0.5));
    water_material(right_sphere.mut_props());
    world.objects.push(Box::new(right_sphere));

    let mut front_sphere = Sphere::new();
    front_sphere
        .mut_props()
        .set_transform(translation(0., -0.51, -2.5) * scaling(0.5, 0.5, 0.5));
    glass_material(front_sphere.mut_props());
    world.objects.push(Box::new(front_sphere));

    let mut back_sphere = Sphere::new();
    back_sphere
        .mut_props()
        .set_transform(translation(0., -0.51, 2.5) * scaling(0.5, 0.5, 0.5));
    mirror_material(back_sphere.mut_props());
    world.objects.push(Box::new(back_sphere));

    // CAMERA SETUP
    let camera = SceneCamera::new(Tuple::point(-0., 0.5, -7.5), Tuple::point(0., 0., 0.));
    let date = date_ymd_his();
    let w = Arc::new(world);

    let start = Instant::now();
    println!("start tracing");

    let canvas = camera.get_camera().render_multi_threaded(w);
    save("cube", canvas, 1, &date);

    println!(
        "finished tracing after {:02} seconds",
        start.elapsed().as_secs_f64()
    );
}
