use crate::intersection::{Intersection, Intersections};
use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shape::{Shape, ShapeProps};
use crate::tuple::Tuple;

#[derive(Debug)]
pub(crate) struct Plane {
    props: ShapeProps,
}

impl Plane {
    pub(crate) fn new() -> Plane {
        Plane {
            props: ShapeProps::default(),
        }
    }
}

impl Shape for Plane {
    fn get_props(&self) -> &ShapeProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut ShapeProps {
        &mut self.props
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::direction(0., 1., 0.)
    }

    fn local_intersect(&self, ray: Ray) -> Option<Intersections> {
        if ray.direction.y.abs() < EPSILON {
            return None;
        }
        let mut xs = Intersections::new();
        let t = -ray.origin.y / ray.direction.y;

        xs.push(Intersection::new(t, self));

        Some(xs)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl From<&dyn Shape> for Plane {
    fn from(shape: &dyn Shape) -> Self {
        Plane {
            props: shape.get_props().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn test_normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();

        let n1 = p.local_normal_at(Tuple::point(0., 0., 0.));
        let n2 = p.local_normal_at(Tuple::point(10., 0., -10.));
        let n3 = p.local_normal_at(Tuple::point(-5., 0., 150.));

        let e = Tuple::direction(0., 1., 0.);

        assert_eq!(e, n1);
        assert_eq!(e, n2);
        assert_eq!(e, n3);
    }

    #[test]
    fn test_intersect_with_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::direction(0., 0., 1.));
        let xs = p.local_intersect(r);

        assert!(xs.is_none());
    }

    #[test]
    fn test_intersect_with_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 0., 1.));
        let xs = p.local_intersect(r);

        assert!(xs.is_none());
    }

    #[test]
    fn test_intersect_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::direction(0., -1., 0.));
        let xs = p.local_intersect(r).unwrap();

        assert_eq!(xs.get(0).unwrap().t, 1.);
        assert_eq!(p, xs.get(0).unwrap().object.into());
    }

    #[test]
    fn test_intersect_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::direction(0., 1., 0.));
        let xs = p.local_intersect(r).unwrap();

        assert_eq!(xs.get(0).unwrap().t, 1.);
        assert_eq!(p, xs.get(0).unwrap().object.into());
    }
}
