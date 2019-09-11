use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::matrix::inverse;

#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> std::cmp::PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}

impl<'a> Intersection<'a> {
    fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub struct Intersections<'a> {
    items: Vec<Intersection<'a>>
}

impl<'a> Intersections<'a> {
    fn new() -> Intersections<'a> {
        Intersections { items: vec![] }
    }

    fn add(&mut self, i: Intersection<'a>) {
        self.items.push(i)
    }

    pub fn get(&self, i: usize) -> Option<&Intersection> {
        Some(&self.items[i])
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

pub fn intersect<'a>(sphere: &'a Sphere, ray: &Ray) -> Option<Intersections<'a>> {
    let ray = ray.transform(inverse(sphere.transformation())?);

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
    let mut xs = Intersections::new();
    xs.add(i1);
    xs.add(i2);

    Some(xs)
}

pub fn hit<'a>(xs: &'a Intersections) -> Option<&'a Intersection<'a>> {
    if let None = xs.items.first() {
        return None;
    }
    let mut lowest = xs.items.first().unwrap();


    for intersection in xs.items.iter() {
        let t = intersection.t;

        if t > 0_f64 && lowest.t < 0_f64 {
            lowest = intersection;
        }

        if t > 0_f64 && t < lowest.t {
            lowest = intersection;
        }
    }

    if lowest.t == -1_f64 {
        None
    } else {
        Some(lowest)
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::{hit, intersect, Intersection, Intersections};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(4.0, xs.items[0].t);
        assert_eq!(6.0, xs.items[1].t);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(5.0, xs.items[0].t);
        assert_eq!(5.0, xs.items[1].t);
    }

    #[test]
    fn a_ray_missing_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = intersect(&s, &r);

        assert!(xs.is_none())
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-1.0, xs.items[0].t);
        assert_eq!(1.0, xs.items[1].t);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = intersect(&s, &r).unwrap();

        assert_eq!(-6.0, xs.items[0].t);
        assert_eq!(-4.0, xs.items[1].t);
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s)
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.items[0].t, 1.0);
        assert_eq!(xs.items[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0),
        ).unwrap();
        let sphere = Sphere::new();
        let xs = intersect(&sphere, &ray).unwrap();

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.items[0].object, &sphere);
        assert_eq!(xs.items[1].object, &sphere)
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);

        let actual = hit(&xs).unwrap();
        assert_eq!(actual, &Intersection::new(1.0, &sphere))
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);

        let actual = hit(&xs).unwrap();
        assert_eq!(actual, &Intersection::new(1.0, &sphere))
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(-2.0, &sphere);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);

        let actual = hit(&xs);
        assert!(actual.is_none())
    }

    #[test]
    fn hit_is_always_lowest_non_negative_value() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4);

        let actual = hit(&xs).unwrap();
        assert_eq!(actual, &Intersection::new(2.0, &sphere))
    }
}