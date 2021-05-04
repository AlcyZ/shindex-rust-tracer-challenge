use crate::intersection::{Intersection, Intersections};
use crate::ray::Ray;
use crate::shape::{Shape, ShapeProps};
use crate::tuple::Tuple;

#[derive(Debug)]
pub(crate) struct Sphere {
    props: ShapeProps,
}

impl Shape for Sphere {
    fn get_props(&self) -> &ShapeProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut ShapeProps {
        &mut self.props
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        point - Tuple::point(0., 0., 0.)
    }

    fn local_intersect(&self, ray: Ray) -> Option<Intersections> {
        let sphere_origin = Tuple::point(0., 0., 0.);
        let sphere_to_ray = ray.origin - sphere_origin;

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        let mut xs = Intersections::new();

        if t1 < t2 {
            xs.push(Intersection::new(t1, self));
            xs.push(Intersection::new(t2, self));
        } else {
            xs.push(Intersection::new(t2, self));
            xs.push(Intersection::new(t1, self));
        }

        Some(xs)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Sphere {
    pub(crate) fn new() -> Sphere {
        Sphere {
            props: ShapeProps::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn test_ray_intersect_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(r).unwrap();

        assert_eq!(4., xs.get(0).unwrap().t);
        assert_eq!(6., xs.get(1).unwrap().t);
    }

    #[test]
    fn test_ray_intersect_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(r).unwrap();

        assert_eq!(5., xs.get(0).unwrap().t);
        assert_eq!(5., xs.get(1).unwrap().t);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let r = s.local_intersect(r);

        assert!(r.is_none());
    }

    #[test]
    fn test_ray_originates_in_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(r).unwrap();

        assert_eq!(-1., xs.get(0).unwrap().t);
        assert_eq!(1., xs.get(1).unwrap().t);
    }

    #[test]
    fn test_ray_is_behind_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(r).unwrap();

        assert_eq!(-6., xs.get(0).unwrap().t);
        assert_eq!(-4., xs.get(1).unwrap().t);
    }

    #[test]
    fn test_intersect_sets_object_on_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(r).unwrap();

        assert_eq!(s.get_id(), xs.get(0).unwrap().object.get_id());
        assert_eq!(s.get_id(), xs.get(1).unwrap().object.get_id());
    }

    #[test]
    fn test_normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(Tuple::direction(1., 0., 0.), n);
    }

    #[test]
    fn test_normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(Tuple::direction(0., 1., 0.), n);
    }

    #[test]
    fn test_normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(Tuple::direction(0., 0., 1.), n);
    }

    #[test]
    fn test_normal_on_sphere_at_non_axial_point() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::point(
            3f64.sqrt() / 3.,
            3f64.sqrt() / 3.,
            3f64.sqrt() / 3.,
        ));

        assert_eq!(
            Tuple::direction(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.),
            n
        );
    }
}
