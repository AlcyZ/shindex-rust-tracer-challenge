use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug)]
struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }
}

pub struct Intersections<'a> {
    items: [Intersection<'a>; 2]
}

impl<'a> Intersections<'a> {
    fn new(i1: Intersection<'a>, i2: Intersection<'a>) -> Intersections<'a> {
        Intersections { items: [i1, i2] }
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

pub fn intersect<'a>(sphere: &'a Sphere, ray: &Ray) -> Option<Intersections<'a>> {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    let i1 = Intersection::new(t1, &sphere);
    let i2 = Intersection::new(t2, &sphere);
    let mut intersections = Intersections::new(i1, i2);

    Some(intersections)
}

#[cfg(test)]
mod tests {
    use crate::intersection::{intersect, Intersection, Intersections};
    use crate::ray::Ray;
    use crate::sphere::sphere;
    use crate::tuple::Tuple;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(4.0, xs.items[0].t);
        assert_eq!(6.0, xs.items[1].t);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(5.0, xs.items[0].t);
        assert_eq!(5.0, xs.items[1].t);
    }

    #[test]
    fn a_ray_missing_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r);

        assert!(xs.is_none())
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-1.0, xs.items[0].t);
        assert_eq!(1.0, xs.items[1].t);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-6.0, xs.items[0].t);
        assert_eq!(-4.0, xs.items[1].t);
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s)
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let mut xs = Intersections::new(i1, i2);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.items[0].t, 1.0);
        assert_eq!(xs.items[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0),
        ).unwrap();
        let sphere = sphere();
        let xs = intersect(&sphere, &ray).unwrap();

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.items[0].object, &sphere);
        assert_eq!(xs.items[1].object, &sphere)
    }
}