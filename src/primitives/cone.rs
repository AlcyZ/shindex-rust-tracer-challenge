use crate::math::tuple::Tuple;
use crate::primitives::shape::{Shape, ShapeProps};
use crate::scene::tracing::intersection::{Intersections, Intersection};
use crate::scene::tracing::ray::Ray;
use crate::math::f64_eq;

#[derive(Debug)]
pub(crate) struct Cone {
    props: ShapeProps,
    min: f64,
    max: f64,
    closed: bool,
}

impl Cone {
    pub(crate) fn new() -> Cone {
        Cone {
            props: ShapeProps::default(),
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }

    pub(crate) fn close(&mut self) {
        self.closed = true;
    }
}

impl Shape for Cone {
    fn get_props(&self) -> &ShapeProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut ShapeProps {
        &mut self.props
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        todo!()
    }

    fn local_intersect(&self, ray: Ray) -> Option<Intersections> {
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2. * ray.origin.x * ray.direction.x - 2. * ray.origin.y * ray.direction.y + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        if f64_eq(a, 0.) {
            if f64_eq(b, 0.) {
                return None;
            }
            let mut xs = Intersections::new();
            xs.push(Intersection::new(-c / (2. * b), self));

            return Some(xs);
        }


        let disc = b.powi(2) - 4. * a * c;

        // ray does not intersect the cone
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

        // xs.merge(self.intersect_caps(ray));

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::f64_eq;

    fn assert_cone_ray_intersection(origin: Tuple, direction: Tuple, t0: f64, t1: f64) {
        let shape = Cone::new();
        let r = Ray::new(origin, direction.normalize());
        let xs = shape.local_intersect(r).unwrap();

        assert!(f64_eq(t0, xs.first().unwrap().t));
        assert!(f64_eq(t1, xs.last().unwrap().t));
    }

    #[test]
    fn test_intersect_cone_with_ray() {
        let cases = [
            (
                Tuple::point(0., 0., -5.),
                Tuple::direction(0., 0., 1.),
                5.,
                5.,
            ),
            (
                Tuple::point(0., 0., -5.),
                Tuple::direction(1., 1., 1.),
                8.66025,
                8.66025,
            ),
            (
                Tuple::point(1., 1., -5.),
                Tuple::direction(-0.5, -1., 1.),
                4.55006,
                49.44994,
            ),
        ];

        for (origin, direction, t0, t1) in cases.iter() {
            assert_cone_ray_intersection(*origin, *direction, *t0, *t1);
        }
    }

    #[test]
    fn test_intersecting_cone_with_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::new();
        let direction = Tuple::direction(0., 1., 1.).normalize();
        let r = Ray::new(Tuple::point(0., 0., -1.), direction);

        let xs = shape.local_intersect(r).unwrap();

        assert_eq!(1, xs.len());
        assert!(f64_eq(0.35355, xs.first().unwrap().t));
    }
}
