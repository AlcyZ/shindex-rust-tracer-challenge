use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::light::PointLight;
use crate::math::matrix::M4;
use crate::math::transformation::{rotation_z, translation, view_transform, rotation_x, rotation_y};
use crate::pattern::checker::CheckerPattern;
use crate::plane::Plane;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::world::World;
use chrono::{Datelike, Local, Timelike};
use std::f64::consts::PI;
use std::sync::Arc;

const SURFACE_WIDTH: usize = 1920;
const SURFACE_HEIGHT: usize = 1080;

struct SceneCameraTransform {
    x: f64,
    y: f64,
    z: f64,
}

struct SceneCamera {
    transform: SceneCameraTransform,
    look_at: SceneCameraTransform,
}

impl SceneCamera {
    fn new(position: Tuple, look_at: Tuple) -> SceneCamera {
        SceneCamera {
            transform: SceneCameraTransform {
                x: position.x,
                y: position.y,
                z: position.z,
            },
            look_at: SceneCameraTransform {
                x: look_at.x,
                y: look_at.y,
                z: look_at.z,
            },
        }
    }

    fn move_camera(&mut self, x: f64, y: f64, z: f64) {
        self.transform.x += x;
        self.transform.y += y;
        self.transform.z += z;
    }

    fn get_camera(&self) -> Camera {
        let mut c = Camera::new(SURFACE_WIDTH, SURFACE_HEIGHT, PI / 3.);
        c.transform = view_transform(
            Tuple::point(self.transform.x, self.transform.y, self.transform.z),
            Tuple::point(self.look_at.x, self.look_at.y, self.look_at.z),
            Tuple::direction(0., 1., 0.),
        );

        c
    }
}

pub fn run() {
    let mut world = World::new();
    world.light = Some(PointLight::new(
        Tuple::point(-10., 5., -10.),
        Color::white(),
    ));

    let mut center_sphere = Sphere::new();
    center_sphere.mut_props().set_material_color(Color::white());
    center_sphere.mut_props().set_material_reflective(1.);
    center_sphere.mut_props().set_material_diffuse(0.);
    world.objects.push(Box::new(center_sphere));

    let mut floor = Plane::new();
    floor.mut_props().set_transform(translation(0., -1., 0.));
    floor.mut_props().set_pattern(Box::new(CheckerPattern::new(
        Color::new(0.2, 0.2, 0.2),
        Color::new(0.8, 0.8, 0.8),
    )));
    floor.mut_props().set_material_reflective(0.1);
    world.objects.push(Box::new(floor));

    let mut right_wall = Plane::new();
    right_wall
        .mut_props()
        .set_material_color(Color::new(0.0, 0.8, 0.1));
    right_wall
        .mut_props()
        .set_pattern(Box::new(CheckerPattern::new(
            Color::new(0.0, 0.5, 0.0),
            Color::new(0.8, 0.8, 0.8),
        )));

    // let mut right_wall_transform = rotation_z(PI / 2.);
    //
    // right_wall_transform = right_wall_transform * translation(5., 0., 0.);

    let mut right_wall_transform = translation(5., 0., 0.);
    right_wall_transform = right_wall_transform * rotation_z(PI / 2.);


    right_wall.mut_props().set_transform(right_wall_transform);
    world.objects.push(Box::new(right_wall));

    let mut camera = SceneCamera::new(Tuple::point(0., 2., -5.), Tuple::point(0., 0., 0.));
    let date = date_ymd_his();
    let w = Arc::new(world);

    for foo in 0..1 {
        camera.move_camera(0., 0.1, 0.);

        let w = w.clone();
        let canvas = camera.get_camera().render_multi_threaded(w);
        save(canvas, foo + 1, &date);
    }
}

fn save(canvas: Canvas, image: usize, date: &str) {
    let destination = format!(
        "./dist/{date}-sequence_{number}.ppm",
        date = date,
        number = image
    );

    std::fs::write(destination, canvas.to_ppm()).unwrap();
}

fn date_ymd_his() -> String {
    let utc = Local::now();
    format!(
        "{year}-{month:02}-{day:02}_{hour:02}-{min:02}-{sec:02}",
        year = utc.year(),
        month = utc.month(),
        day = utc.day(),
        hour = utc.hour(),
        min = utc.minute(),
        sec = utc.second()
    )
}
