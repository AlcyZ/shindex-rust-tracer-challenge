use std::sync::{Arc, mpsc};
use std::sync::mpsc::SendError;

use crate::canvas::Canvas;
use crate::matrix::{inverse, Matrix4x4, MATRIX_4X4_IDENTITY, mul_by_tuple};
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

#[derive(Clone)]
pub struct Camera {
    h_size: usize,
    v_size: usize,
    field_of_view: f64,
    transform: Matrix4x4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Camera {
        let (pixel_size, half_width, half_height) = Camera::calculate_init(h_size, v_size, field_of_view);

        Camera { h_size, v_size, field_of_view, transform: MATRIX_4X4_IDENTITY, pixel_size, half_width, half_height }
    }

    fn calculate_init(h_size: usize, v_size: usize, field_of_view: f64) -> (f64, f64, f64) {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = h_size as f64 / v_size as f64;
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / h_size as f64;

        (pixel_size, half_width, half_height)
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = mul_by_tuple(inverse(self.transform).unwrap(), Tuple::point(world_x, world_y, -1.0));
        let origin = mul_by_tuple(inverse(self.transform).unwrap(), Tuple::origin_point());
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction).unwrap()
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);

        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }

    pub fn transform(&mut self, transformation: Matrix4x4) {
        self.transform = transformation;
    }
}

pub fn render(camera: Camera, world: Arc<World>) -> Canvas {
    let mut image = Canvas::new(camera.h_size, camera.v_size);
    let (tx, rx) = mpsc::channel();

    for y in 0..camera.v_size {
        let tx_c = mpsc::Sender::clone(&tx);
        let cam_clone = camera.clone();
        let world_clone = world.clone();

        std::thread::spawn(move || {
            for x in 0..cam_clone.h_size {
                let ray = cam_clone.ray_for_pixel(x, y);
                let color = world_clone.color_at(&ray);

                tx_c.send(Some((x, y, color))).unwrap();
            }
        });
    }
    std::thread::spawn(move || tx.send(None));

    for rec in rx {
        if let Some((x, y, color)) = rec {
            image.write_pixel(x, y, color);
        }
    }

    image
}

    #[cfg(test)]
    mod test {
    use std::f64::consts::PI;
    use std::sync::Arc;

    use crate::camera::Camera;
    use crate::color::Color;
    use crate::matrix::{MATRIX_4X4_IDENTITY, mul};
    use crate::transformation::{rotation_y, translation, view_transform};
    use crate::tuple::Tuple;
    use crate::util::f64_eq;
    use crate::world::test_default_world;

    #[test]
    fn constructing_camera() {
        let h_size = 160;
        let v_size = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(h_size, v_size, field_of_view);

        assert_eq!(c.h_size, h_size);
        assert_eq!(c.v_size, v_size);
        assert_eq!(c.field_of_view, field_of_view);
        assert_eq!(c.transform, MATRIX_4X4_IDENTITY);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01), "c.pixel_size: {}", c.pixel_size);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::origin_point());
        assert!(f64_eq(r.direction.x, Tuple::vector(0.0, 0.0, -1.0).x));
        assert!(f64_eq(r.direction.y, Tuple::vector(0.0, 0.0, -1.0).y));
        assert!(f64_eq(r.direction.z, Tuple::vector(0.0, 0.0, -1.0).z));
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::origin_point());
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851))
    }

    #[test]
    fn constructing_ray_then_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = mul(rotation_y(PI / 4.0), translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);

        let a = 2_f64.sqrt() / 2.0;
        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Tuple::vector(a, 0.0, -a))
    }

    #[test]
    fn rendering_a_world_with_camera() {
        let w = test_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::origin_point();
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);
        let image = c.render(&w);

        assert_eq!(*image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855))
    }
}
