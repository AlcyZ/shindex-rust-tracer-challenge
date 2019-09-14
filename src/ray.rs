use crate::matrix::{Matrix4x4, mul_by_tuple};
use crate::ray::RayError::{DirectionIsNotVector, OriginIsNotPoint};
use crate::transformation::{scaling, translation};
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[derive(Debug)]
pub enum RayError {
    OriginIsNotPoint,
    DirectionIsNotVector,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Result<Ray, RayError> {
        if !origin.is_point() {
            return Err(RayError::OriginIsNotPoint);
        }
        if !direction.is_vector() {
            return Err(RayError::DirectionIsNotVector);
        }

        Ok(Ray { origin, direction })
    }

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray::new(mul_by_tuple(m, self.origin), mul_by_tuple(m, self.direction)).unwrap()
    }

    pub fn position (&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }
}


#[cfg(test)]
mod tests {
    use crate::ray::{Ray};
    use crate::transformation::{scaling, translation};
    use crate::tuple::Tuple;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction).expect("wrong initial values");

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction)
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0)).expect("wrong init values");

        let expected_a = Tuple::point(2.0, 3.0, 4.0);
        let expected_b = Tuple::point(3.0, 3.0, 4.0);
        let expected_c = Tuple::point(1.0, 3.0, 4.0);
        let expected_d = Tuple::point(4.5, 3.0, 4.0);

        assert_eq!(position(&r, 0.0), expected_a);
        assert_eq!(position(&r, 1.0), expected_b);
        assert_eq!(position(&r, -1.0), expected_c);
        assert_eq!(position(&r, 2.5), expected_d);
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0)).unwrap();
        let r2 = r.transform(translation(3.0, 4.0, 5.0));

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }


    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0)).unwrap();
        let r2 = r.transform(scaling(2.0, 3.0, 4.0));

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }
}
