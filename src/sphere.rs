use crate::ray::Ray;
use crate::tuple::{dot, point, tuple_subtract};

struct Sphere {}

fn intersect(sphere: &Sphere, ray: &Ray) -> Option<[f64; 2]> {
    let sphere_to_ray = tuple_subtract(ray.origin, point(0.0, 0.0, 0.0));

    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(ray.direction, sphere_to_ray);
    let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    Some([t1, t2])
}

fn sphere() -> Sphere {
    Sphere {}
}


#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::sphere::{intersect, sphere};
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