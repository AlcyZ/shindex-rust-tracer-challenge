use crate::ray::RayError::{DirectionIsNotVector, OriginIsNotPoint};
use crate::tuple::{Tuple, tuple_is_point, tuple_is_vector};

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

#[derive(Debug)]
enum RayError {
    OriginIsNotPoint,
    DirectionIsNotVector,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Result<Ray, RayError> {
        if !tuple_is_point(origin) {
            return Err(RayError::OriginIsNotPoint);
        }
        if !tuple_is_vector(direction) {
            return Err(RayError::DirectionIsNotVector);
        }

        Ok(Ray { origin, direction })
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::tuple::{point, vector};

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = vector(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction).expect("wrong initial values");

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction)
    }
}
