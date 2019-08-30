use crate::ray::Ray;
use crate::tuple::{dot, point, tuple_subtract};

pub struct Sphere {}

fn sphere() -> Sphere {
    Sphere {}
}


#[cfg(test)]
mod tests {
    use crate::intersection::intersect;
    use crate::ray::Ray;
    use crate::sphere::sphere;
    use crate::tuple::{point, vector};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
    }

    #[test]
    fn a_ray_missing_a_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r);

        assert!(xs.is_none())
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }
}