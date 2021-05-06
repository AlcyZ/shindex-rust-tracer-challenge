use crate::math::matrix::M4;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ray {
    pub(crate) origin: Tuple,
    pub(crate) direction: Tuple,
}

impl Ray {
    pub(crate) fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    pub(crate) fn transform(&self, m: M4) -> Ray {
        Ray::new(m * self.origin, m * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::transformation::{scaling, translation};
    use crate::tuple::Tuple;

    #[test]
    fn test_create_and_query_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::direction(4., 5., 6.);

        let ray = Ray::new(origin, direction);

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn test_compute_point_from_distance() {
        let ray = Ray::new(Tuple::point(2., 3., 4.), Tuple::direction(1., 0., 0.));

        assert_eq!(Tuple::point(2., 3., 4.), ray.position(0.));
        assert_eq!(Tuple::point(3., 3., 4.), ray.position(1.));
        assert_eq!(Tuple::point(1., 3., 4.), ray.position(-1.));
        assert_eq!(Tuple::point(4.5, 3., 4.), ray.position(2.5));
    }

    #[test]
    fn test_translating_a_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::direction(0., 1., 0.));
        let m = translation(3., 4., 5.);

        let r2 = ray.transform(m);

        assert_eq!(r2.origin, Tuple::point(4., 6., 8.));
        assert_eq!(r2.direction, Tuple::direction(0., 1., 0.));
    }

    #[test]
    fn test_scaling_a_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::direction(0., 1., 0.));
        let m = scaling(2., 3., 4.);

        let r2 = ray.transform(m);

        assert_eq!(r2.origin, Tuple::point(2., 6., 12.));
        assert_eq!(r2.direction, Tuple::direction(0., 3., 0.));
    }
}
