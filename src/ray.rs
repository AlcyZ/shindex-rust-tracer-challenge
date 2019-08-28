use crate::ray::RayError::{DirectionIsNotVector, OriginIsNotPoint};
use crate::tuple::{Tuple, tuple_add, tuple_is_point, tuple_is_vector, tuple_mul_scalar};

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
        if !tuple_is_point(origin) {
            return Err(RayError::OriginIsNotPoint);
        }
        if !tuple_is_vector(direction) {
            return Err(RayError::DirectionIsNotVector);
        }

        Ok(Ray { origin, direction })
    }
}

fn position(r: &Ray, time: f64) -> Tuple {
    tuple_add(r.origin, tuple_mul_scalar(r.direction, time))
}

#[cfg(test)]
mod tests {
    use crate::ray::{position, Ray};
    use crate::tuple::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction).expect("wrong initial values");

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction)
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0)).expect("wrong init values");

        let expected_a = point(2.0, 3.0, 4.0);
        let expected_b = point(3.0, 3.0, 4.0);
        let expected_c = point(1.0, 3.0, 4.0);
        let expected_d = point(4.5, 3.0, 4.0);

        assert_eq!(position(&r, 0.0), expected_a);
        assert_eq!(position(&r, 1.0), expected_b);
        assert_eq!(position(&r, -1.0), expected_c);
        assert_eq!(position(&r, 2.5), expected_d);
    }
}
