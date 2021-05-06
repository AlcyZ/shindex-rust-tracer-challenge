use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::math::transformation::view_transform;
use crate::tuple::Tuple;
use chrono::{Datelike, Local, Timelike};
use std::f64::consts::PI;
use image::io::Reader;
use std::io::Cursor;

const SURFACE_WIDTH: usize = 1920;
const SURFACE_HEIGHT: usize = 1080;
// const SURFACE_WIDTH: usize = 640;
// const SURFACE_HEIGHT: usize = 480;
// const SURFACE_WIDTH: usize = 320;
// const SURFACE_HEIGHT: usize = 240;

pub(super) fn save(name: &str, canvas: Canvas, image: usize, date: &str) {
    let destination = format!(
        "./dist/{date}-{name}_{number}.png",
        date = date,
        name = name,
        number = image
    );

    let reader = Reader::new(Cursor::new(canvas.to_ppm())).with_guessed_format().expect("Cursor io never fails");
    let img = reader.decode().unwrap();

    img.save(destination).unwrap()
    // std::fs::write(destination, canvas.to_ppm()).unwrap();
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

    pub(super) fn move_camera(&mut self, x: f64, y: f64, z: f64) {
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
