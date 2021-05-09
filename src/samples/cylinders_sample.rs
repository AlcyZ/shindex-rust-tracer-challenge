use crate::math::transformation::{rotation_y, scaling, translation};
use crate::math::tuple::Tuple;
use crate::primitives::cube::Cube;
use crate::primitives::cylinder::Cylinder;
use crate::primitives::shape::Shape;
use crate::primitives::sphere::Sphere;
use crate::samples::utility::{
    date_ymd_his, glass_material, mirror_material, save, water_material, SceneCamera,
};
use crate::scene::shading::color::Color;
use crate::scene::shading::light::PointLight;
use crate::scene::world::World;
use std::f64::consts::PI;
use std::sync::Arc;

pub fn _run() {
    let mut world = World::new();
    world.light = Some(PointLight::new(Tuple::point(2., 5., -5.), Color::white()));

    // WORLD POPULATION
    let mut room = Cube::new();
    room.mut_props().set_transform(
        translation(0., 6.49, 0.) * scaling(7.5, 7.5, 7.5), /* * rotation_y(PI / 4.)*/
    );
    room.mut_props()
        .set_material_color(Color::new(0.4, 0.2, 0.2));
    room.mut_props().set_material_reflective(0.2);
    world.objects.push(Box::new(room));

    let mut cylinder = Cylinder::with_min_max(-1., 1.);
    cylinder.close();
    mirror_material(cylinder.mut_props());
    world.objects.push(Box::new(cylinder));

    let mut cube_behind = Cube::new();
    cube_behind
        .mut_props()
        .set_transform(translation(0., -0.25, 2.25) * scaling(0.75, 0.75, 0.75));
    glass_material(cube_behind.mut_props());
    world.objects.push(Box::new(cube_behind));

    let mut cube_front = Cube::new();
    cube_front
        .mut_props()
        .set_transform(translation(0., -0.25, -2.25) * scaling(0.75, 0.75, 0.75));
    glass_material(cube_front.mut_props());
    world.objects.push(Box::new(cube_front));

    let mut left_sphere = Sphere::new();
    left_sphere
        .mut_props()
        .set_transform(translation(-2.25, -0.25, 0.) * scaling(0.75, 0.75, 0.75));
    mirror_material(left_sphere.mut_props());
    world.objects.push(Box::new(left_sphere));

    let mut right_sphere = Sphere::new();
    right_sphere
        .mut_props()
        .set_transform(translation(2.25, -0.25, 0.) * scaling(0.75, 0.75, 0.75));
    mirror_material(right_sphere.mut_props());
    world.objects.push(Box::new(right_sphere));

    // CAMERA SETUP AND RENDERING
    let camera_position = Tuple::point(4., 4., -4.);
    let camera = SceneCamera::new(camera_position, Tuple::point(0., 0., 0.));
    let canvas = camera.get_camera().render_multi_threaded(Arc::new(world));
    let date = date_ymd_his();
    save("cylinders_sample", canvas, 1, &date);
}
