use crate::intersection::{Intersection, Intersections};
use crate::math::{f64_eq, EPSILON};
use crate::ray::Ray;
use crate::shape::{Shape, ShapeProps};
use crate::tuple::Tuple;
use std::cmp::{max, min};
use std::f64::INFINITY;

#[derive(Debug)]
pub(crate) struct Cube {
    props: ShapeProps,
}

impl Cube {
    pub(crate) fn new() -> Cube {
        Cube {
            props: ShapeProps::default(),
        }
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let t_min_numerator = (-1. - origin);
        let t_max_numerator = (1. - origin);

        let (t_min, t_max) = if direction.abs() >= EPSILON {
            (t_min_numerator / direction, t_max_numerator / direction)
        } else {
            (t_min_numerator * INFINITY, t_max_numerator * INFINITY)
        };

        if t_min > t_max {
            return (t_max, t_min);
        }

        (t_min, t_max)
    }
}

impl Shape for Cube {
    fn get_props(&self) -> &ShapeProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut ShapeProps {
        &mut self.props
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let max_c = point.x.abs().max(point.y.abs().max(point.z.abs()));

        if f64_eq(max_c, point.x.abs()) {
            return Tuple::direction(point.x, 0., 0.);
        } else if f64_eq(max_c, point.y.abs()) {
            return Tuple::direction(0., point.y, 0.);
        }

        Tuple::direction(0., 0., point.z)
    }

    fn local_intersect(&self, ray: Ray) -> Option<Intersections> {
        let (x_min, x_max) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (y_min, y_max) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (z_min, z_max) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let t_min = x_min.max(y_min.max(z_min));
        let t_max = x_max.min(y_max.min(z_max));

        if t_min > t_max {
            return None;
        }

        let mut xs = Intersections::new();
        xs.push(Intersection::new(t_min, self));
        xs.push(Intersection::new(t_max, self));

        Some(xs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    fn test_ray_cube_intersection(origin: Tuple, direction: Tuple, t1: f64, t2: f64) {
        let c = Cube::new();
        let r = Ray::new(origin, direction);

        let xs = c.local_intersect(r).unwrap();

        assert_eq!(t1, xs.first().unwrap().t);
        assert_eq!(t2, xs.last().unwrap().t);
    }

    fn test_ray_cube_miss(origin: Tuple, direction: Tuple) {
        let c = Cube::new();
        let r = Ray::new(origin, direction);

        let xs = c.local_intersect(r);

        assert!(xs.is_none());
    }

    fn test_ray_normal(point: Tuple, normal: Tuple) {
        let c = Cube::new();
        let actual = c.local_normal_at(point);

        assert_eq!(normal, actual);
    }

    #[test]
    fn test_ray_intersects_cube() {
        let cases = [
            // x
            (
                Tuple::point(5., 0.5, 0.),
                Tuple::direction(-1., 0., 0.),
                4.,
                6.,
            ),
            (
                Tuple::point(-5., 0.5, 0.),
                Tuple::direction(1., 0., 0.),
                4.,
                6.,
            ),
            // y
            (
                Tuple::point(0.5, 5., 0.),
                Tuple::direction(0., -1., 0.),
                4.,
                6.,
            ),
            (
                Tuple::point(0.5, -5., 0.),
                Tuple::direction(0., 1., 0.),
                4.,
                6.,
            ),
            // z
            (
                Tuple::point(0.5, 0., 5.),
                Tuple::direction(0., 0., -1.),
                4.,
                6.,
            ),
            (
                Tuple::point(0.5, 0., -5.),
                Tuple::direction(0., 0., 1.),
                4.,
                6.,
            ),
            // inside
            (
                Tuple::point(0., 0.5, 0.),
                Tuple::direction(0., 0., 1.),
                -1.,
                1.,
            ),
        ];

        for (origin, direction, t1, t2) in cases.iter() {
            test_ray_cube_intersection(*origin, *direction, *t1, *t2);
        }
    }

    #[test]
    fn test_ray_misses_cube() {
        let cases = [
            (
                Tuple::point(-2., 0., 0.),
                Tuple::direction(0.2673, 0.5345, 0.8018),
            ),
            (
                Tuple::point(0., -2., 0.),
                Tuple::direction(0.8018, 0.2673, 0.5345),
            ),
            (
                Tuple::point(0., 0., -2.),
                Tuple::direction(0.5345, 0.8018, 0.2673),
            ),
            (Tuple::point(2., 0., 2.), Tuple::direction(0., 0., -1.)),
            (Tuple::point(0., 2., 2.), Tuple::direction(0., -1., 0.)),
            (Tuple::point(2., 2., 0.), Tuple::direction(-1., 0., 0.)),
        ];

        for (origin, direction) in cases.iter() {
            test_ray_cube_miss(*origin, *direction);
        }
    }

    #[test]
    fn test_normal_on_surface_of_cube() {
        let cases = [
            (Tuple::point(1., 0.5, -0.8), Tuple::direction(1., 0., 0.)),
            (Tuple::point(-1., -0.2, 0.9), Tuple::direction(-1., 0., 0.)),
            (Tuple::point(-0.4, 1., -0.1), Tuple::direction(0., 1., 0.)),
            (Tuple::point(0.3, -1., -0.7), Tuple::direction(0., -1., 0.)),
            (Tuple::point(-0.6, 0.3, 1.), Tuple::direction(0., 0., 1.)),
            (Tuple::point(0.4, 0.4, -1.), Tuple::direction(0., 0., -1.)),
            (Tuple::point(1., 1., 1.), Tuple::direction(1., 0., 0.)),
            (Tuple::point(-1., -1., -1.), Tuple::direction(-1., 0., 0.)),
        ];

        for (point, normal) in cases.iter() {
            test_ray_normal(*point, *normal);
        }
    }
}
