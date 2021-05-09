use crate::math::transformation::view_transform;
use crate::math::tuple::Tuple;
use crate::primitives::shape::ShapeProps;
use crate::scene::camera::Camera;
use crate::scene::canvas::Canvas;
use chrono::{Datelike, Local, Timelike};
use std::f64::consts::PI;

// const SURFACE_WIDTH: usize = 1920;
// const SURFACE_HEIGHT: usize = 1080;
// const SURFACE_WIDTH: usize = 640;
// const SURFACE_HEIGHT: usize = 480;
const SURFACE_WIDTH: usize = 320;
const SURFACE_HEIGHT: usize = 240;
// const SURFACE_WIDTH: usize = 32;
// const SURFACE_HEIGHT: usize = 24;

pub(super) fn mirror_material(props: &mut ShapeProps) {
    props.set_material_diffuse(0.1);
    props.set_material_ambient(0.1);
    props.set_material_reflective(1.0);
    props._set_material_shininess(300.);
    props.set_material_specular(1.);
}

pub(super) fn glass_material(props: &mut ShapeProps) {
    props.set_material_diffuse(0.1);
    props.set_material_ambient(0.1);
    props.set_material_reflective(0.9);
    props.set_material_transparency(1.0);
    props.set_material_refractive_index(1.5);
    props._set_material_shininess(300.);
    props.set_material_specular(1.);
}

pub(super) fn water_material(props: &mut ShapeProps) {
    props.set_material_diffuse(0.1);
    props.set_material_ambient(0.1);
    props.set_material_reflective(0.9);
    props.set_material_transparency(1.0);
    props.set_material_refractive_index(1.33);
    props._set_material_shininess(15.);
    props.set_material_specular(0.8);
}

pub(super) fn save(name: &str, canvas: Canvas, image: usize, date: &str) {
    let destination = format!(
        "./dist/{date}-{name}_{number}.ppm",
        date = date,
        name = name,
        number = image
    );

    // let reader = Reader::new(Cursor::new(canvas.to_ppm()))
    //     .with_guessed_format()
    //     .expect("Cursor io never fails");
    // let img = reader.decode().unwrap();
    //
    // img.save(destination).unwrap()
    std::fs::write(destination, canvas.to_ppm()).unwrap();
}

pub(super) fn date_ymd_his() -> String {
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

pub(super) struct SceneCamera {
    transform: SceneCameraTransform,
    look_at: SceneCameraTransform,
}

impl SceneCamera {
    pub(super) fn new(position: Tuple, look_at: Tuple) -> SceneCamera {
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

    pub(super) fn _move_camera(&mut self, x: f64, y: f64, z: f64) {
        self.transform.x += x;
        self.transform.y += y;
        self.transform.z += z;
    }

    pub(super) fn get_camera(&self) -> Camera {
        let mut c = Camera::new(SURFACE_WIDTH, SURFACE_HEIGHT, PI / 3.);
        c.transform = view_transform(
            Tuple::point(self.transform.x, self.transform.y, self.transform.z),
            Tuple::point(self.look_at.x, self.look_at.y, self.look_at.z),
            Tuple::direction(0., 1., 0.),
        );

        c
    }
}

struct SceneCameraTransform {
    x: f64,
    y: f64,
    z: f64,
}
