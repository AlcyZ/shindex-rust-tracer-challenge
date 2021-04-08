use crate::canvas::Canvas;
use crate::math::matrix::M4;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::world::World;

pub(crate) struct Camera {
    h_size: usize,
    v_size: usize,
    fov: f64,
    pub(crate) transform: M4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub(crate) fn new(h_size: usize, v_size: usize, fov: f64) -> Camera {
        let transform = M4::identity();
        let half_view = (fov / 2.).tan();
        let aspect = h_size as f64 / v_size as f64;

        let (half_width, half_height) = {
            if aspect >= 1. {
                (half_view, half_view / aspect)
            } else {
                (half_view * aspect, half_view)
            }
        };
        let pixel_size = (half_width * 2.) / h_size as f64;

        Camera {
            h_size,
            v_size,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub(crate) fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.h_size, self.v_size);

        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.write_pixel(x, y, color);
            }

            println!("finished row: {}", y + 1);
        }

        canvas
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let inverse_transform = self.transform.inverse().unwrap();

        let pixel = inverse_transform * Tuple::point(world_x, world_y, -1.);
        let origin = inverse_transform * Tuple::point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::math::f64_eq;
    use crate::math::transformation::{rotation_y, scaling, translation, view_transform};
    use crate::shape::Shape;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::World;
    use std::f64::consts::PI;

    #[test]
    fn test_create_camera() {
        let h_size = 160;
        let v_size = 120;
        let fov = PI / 2.;

        let c = Camera::new(h_size, v_size, fov);

        assert_eq!(160, c.h_size);
        assert_eq!(120, c.v_size);
        assert_eq!(PI / 2., c.fov);
        assert_eq!(M4::identity(), c.transform);
    }

    #[test]
    fn test_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);

        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn test_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);

        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn test_create_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::direction(0., 0., -1.));
    }

    #[test]
    fn test_create_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_eq!(r.direction, Tuple::direction(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_create_ray_then_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.);
        c.transform = rotation_y(PI / 4.) * translation(0., -2., 5.);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 2., -5.));
        assert_eq!(
            r.direction,
            Tuple::direction(2f64.sqrt() / 2., 0., -2f64.sqrt() / 2.)
        );
    }

    #[test]
    fn test_render_world_with_camera() {
        let point_light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::new();
        s1.mut_props().set_material_color(Color::new(0.8, 1., 0.6));
        s1.mut_props().set_material_diffuse(0.7);
        s1.mut_props().set_material_specular(0.2);

        let mut s2 = Sphere::new();
        s2.mut_props().set_transform(scaling(0.5, 0.5, 0.5));

        let mut w = World::new();
        w.light = Some(point_light);
        w.objects.push(Box::new(s1));
        w.objects.push(Box::new(s2));

        let mut c = Camera::new(11, 11, PI / 2.);
        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::direction(0., 1., 0.);
        c.transform = view_transform(from, to, up);

        let image = c.render(&w);
        assert_eq!(
            Color::new(0.38066, 0.47583, 0.2855),
            image.pixel_at(5, 5).unwrap()
        );
    }
}
