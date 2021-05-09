use crate::math::tuple::Tuple;
use crate::math::{f64_eq, EPSILON};
use crate::primitives::shape::{Shape, ShapeProps};
use crate::scene::tracing::intersection::{Intersection, Intersections};
use crate::scene::tracing::ray::Ray;

#[derive(Debug)]
pub(crate) struct Cylinder {
    props: ShapeProps,
    min: f64,
    max: f64,
    closed: bool,
}

impl Cylinder {
    pub(crate) fn new() -> Cylinder {
        Cylinder {
            props: ShapeProps::default(),
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }

    pub(crate) fn with_min_max(min: f64, max: f64) -> Cylinder {
        Cylinder {
            props: ShapeProps::default(),
            min,
            max,
            closed: false,
        }
    }

    pub(crate) fn close(&mut self) {
        self.closed = true;
    }

    fn intersect_caps(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new();

        // caps only matter if the cylinder is closed, and might possibly be
        // intersected by the ray.
        if !self.closed || f64_eq(ray.direction.y, 0.) {
            return xs;
        }

        // check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cyl.minimum
        let t = (self.min - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        // check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cyl.maximum
        let t = (self.max - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t) {
            xs.push(Intersection::new(t, self));
        }

        xs
    }
}

impl Shape for Cylinder {
    fn get_props(&self) -> &ShapeProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut ShapeProps {
        &mut self.props
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        // compute the square of the distance from the y axis
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1. && point.y >= self.max - EPSILON {
            return Tuple::direction(0., 1., 0.);
        }

        if dist < 1. && point.y <= self.min + EPSILON {
            return Tuple::direction(0., -1., 0.);
        }

        Tuple::direction(point.x, 0., point.z)
    }

    fn local_intersect(&self, ray: Ray) -> Option<Intersections> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        // ray is parallel to the y axis
        if f64_eq(a, 0.) {
            return Some(self.intersect_caps(ray));
        }
        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;
        let disc = b.powi(2) - 4. * a * c;

        // ray does not intersect the cylinder
        if disc < 0. {
            return None;
        }

        let mut xs = Intersections::new();
        let (t0, t1) = min_max((-b - disc.sqrt()) / (2. * a), (-b + disc.sqrt()) / (2. * a));

        let y0 = ray.origin.y + t0 * ray.direction.y;
        if self.min < y0 && y0 < self.max {
            xs.push(Intersection::new(t0, self));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.min < y1 && y1 < self.max {
            xs.push(Intersection::new(t1, self));
        }

        xs.merge(self.intersect_caps(ray));

        Some(xs)
    }
}

fn min_max(a: f64, b: f64) -> (f64, f64) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn check_cap(ray: Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;

    x.powi(2) + z.powi(2) <= 1.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ray_miss(origin: Tuple, direction: Tuple) {
        let cyl = Cylinder::new();
        let r = Ray::new(origin, direction.normalize());

        let xs = cyl.local_intersect(r);

        match xs {
            Some(intersections) => assert_eq!(0, intersections.len()),
            None => (),
        }
    }

    #[test]
    fn test_ray_miss_cylinder() {
        let cases = [
            (Tuple::point(1., 0., 0.), Tuple::direction(0., 1., 0.)),
            (Tuple::point(0., 0., 0.), Tuple::direction(0., 1., 0.)),
            (Tuple::point(0., 0., -5.), Tuple::direction(1., 1., 1.)),
        ];

        for (origin, direction) in cases.iter() {
            assert_ray_miss(*origin, *direction);
        }
    }

    fn assert_ray_strikes(t0: f64, t1: f64, origin: Tuple, direction: Tuple) {
        let cyl = Cylinder::new();
        let r = Ray::new(origin, direction.normalize());

        let xs = cyl.local_intersect(r).unwrap();

        assert!(f64_eq(t0, xs.first().unwrap().t));
        assert!(f64_eq(t1, xs.last().unwrap().t));
    }

    #[test]
    fn test_ray_strikes_cylinder() {
        let cases = [
            (
                Tuple::point(1., 0., -5.),
                Tuple::direction(0., 0., 1.),
                5.,
                5.,
            ),
            (
                Tuple::point(0., 0., -5.),
                Tuple::direction(0., 0., 1.),
                4.,
                6.,
            ),
            (
                Tuple::point(0.5, 0., -5.),
                Tuple::direction(0.1, 1., 1.),
                6.80798,
                7.08872,
            ),
        ];

        for (origin, direction, t0, t1) in cases.iter() {
            assert_ray_strikes(*t0, *t1, *origin, *direction);
        }
    }

    fn assert_normal(point: Tuple, normal: Tuple) {
        let cyl = Cylinder::new();
        let n = cyl.local_normal_at(point);

        assert_eq!(normal, n);
    }

    #[test]
    fn test_normal_vector_on_cylinder() {
        let cases = [
            (Tuple::point(1., 0., 0.), Tuple::direction(1., 0., 0.)),
            (Tuple::point(0., 5., -1.), Tuple::direction(0., 0., -1.)),
            (Tuple::point(0., -2., 1.), Tuple::direction(0., 0., 1.)),
            (Tuple::point(-1., 1., 0.), Tuple::direction(-1., 0., 0.)),
        ];

        for (point, normal) in cases.iter() {
            assert_normal(*point, *normal);
        }
    }

    #[test]
    fn test_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(f64::NEG_INFINITY, cyl.min);
        assert_eq!(f64::INFINITY, cyl.max);
    }

    fn assert_ray_strikes_constrained(origin: Tuple, direction: Tuple, len: usize) {
        let cyl = Cylinder::with_min_max(1., 2.);
        let r = Ray::new(origin, direction.normalize());
        let xs = cyl.local_intersect(r).unwrap();

        assert_eq!(len, xs.len());
    }

    #[test]
    fn test_intersecting_a_constrained_cylinder() {
        let cases = [
            (Tuple::point(0., 1.5, 0.), Tuple::direction(0.1, 1., 0.), 0),
            (Tuple::point(0., 3., -5.), Tuple::direction(0., 0., 1.), 0),
            (Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.), 0),
            (Tuple::point(0., 2., -5.), Tuple::direction(0., 0., 1.), 0),
            (Tuple::point(0., 1., -5.), Tuple::direction(0., 0., 1.), 0),
            (Tuple::point(0., 1.5, -2.), Tuple::direction(0., 0., 1.), 2),
        ];

        for (origin, direction, is_some) in cases.iter() {
            assert_ray_strikes_constrained(*origin, *direction, *is_some);
        }
    }

    #[test]
    fn test_cylinder_is_not_closed_as_default() {
        let cyl = Cylinder::new();

        assert!(!cyl.closed);
    }

    fn assert_ray_strikes_closed_container(origin: Tuple, direction: Tuple) {
        let mut cyl = Cylinder::with_min_max(1., 2.);
        cyl.close();

        let r = Ray::new(origin, direction.normalize());
        let xs = cyl.local_intersect(r).unwrap();

        assert_eq!(2, xs.len());
    }

    #[test]
    fn test_intersecting_caps_of_closed_cylinder() {
        let cases = [
            (Tuple::point(0., 3., 0.), Tuple::direction(0., -1., 0.)),
            (Tuple::point(0., 3., -2.), Tuple::direction(0., -1., 2.)),
            (Tuple::point(0., 4., -2.), Tuple::direction(0., -1., 1.)),
            (Tuple::point(0., 0., -2.), Tuple::direction(0., 1., 2.)),
            (Tuple::point(0., -1., -2.), Tuple::direction(0., 1., 1.)),
        ];

        for (origin, direction) in cases.iter() {
            assert_ray_strikes_closed_container(*origin, *direction);
        }
    }

    fn assert_normal_with_caps(point: Tuple, normal: Tuple) {
        let mut cyl = Cylinder::with_min_max(1., 2.);
        cyl.close();

        let n = cyl.local_normal_at(point);
        assert_eq!(normal, n);
    }

    #[test]
    fn test_normal_vector_on_cylinder_end_caps() {
        let cases = [
            (Tuple::point(0., 1., 0.), Tuple::direction(0., -1., 0.)),
            (Tuple::point(0.5, 1., 0.), Tuple::direction(0., -1., 0.)),
            (Tuple::point(0., 1., 0.5), Tuple::direction(0., -1., 0.)),
            (Tuple::point(0., 2., 0.), Tuple::direction(0., 1., 0.)),
            (Tuple::point(0.5, 2., 0.), Tuple::direction(0., 1., 0.)),
            (Tuple::point(0., 2., 0.5), Tuple::direction(0., 1., 0.)),
        ];

        for (point, normal) in cases.iter() {
            assert_normal_with_caps(*point, *normal);
        }
    }
}
