use std::cmp::Ordering;

use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

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
    pub fn new(t: f64, object: &'a Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn object(&self) -> &'a Sphere {
        self.object
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub struct Intersections<'a> {
    items: Vec<Intersection<'a>>
}

impl<'a> Intersections<'a> {
    pub fn new(first: Intersection<'a>) -> Intersections<'a> {
        Intersections { items: vec![first] }
    }

    pub fn from_intersections(i: Vec<Intersection>) -> Intersections {
        Intersections { items: i }
    }

    pub fn add(&mut self, i: Intersection<'a>) {
        self.items.push(i)
    }

    pub fn get(&self, i: usize) -> Option<&Intersection> {
        Some(&self.items[i])
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn sort(&self) {}

    pub fn hit(&self) -> Option<&Intersection> {
        let mut lowest = self.items.first().unwrap();

        for intersection in self.items.iter() {
            let t = intersection.t;

            if t > 0_f64 && lowest.t < 0_f64 {
                lowest = intersection;
            }

            if t > 0_f64 && t < lowest.t {
                lowest = intersection;
            }
        }

        if lowest.t < 0_f64 {
            None
        } else {
            Some(lowest)
        }
    }
}

pub struct Computation<'a> {
    t: f64,
    object: &'a Sphere,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
}

/// Todo: the normal_at match is a bit strange .. maybe i should take a look at that again and think if  it is possible that normal_at could fail
impl<'a> Computation<'a> {
    pub fn prepare(i: &'a Intersection, r: &Ray) -> Option<Computation<'a>> {
        let point = r.position(i.t);

        match i.object.normal_at(point) {
            Ok(mut normal) => {
                let eye_v = -r.direction;
                let inside;

                if normal.dot(eye_v) < 0.0 {
                    inside = true;
                    normal = -normal;
                } else {
                    inside = false;
                }

                Some(Computation::new(
                    i.t,
                    i.object,
                    point,
                    -r.direction,
                    normal,
                    inside,
                ))
            }
            Err(e) => None
        }
    }

    fn new(t: f64, object: &'a Sphere, point: Tuple, eye_v_: Tuple, normal_v: Tuple, inside: bool) -> Computation {
        Computation { t, object, point, eye_v: eye_v_, normal_v, inside }
    }

    pub fn material(&self) -> &Material {
        self.object.material()
    }
}

#[cfg(test)]
mod tests {
    use crate::intersection::{Computation, Intersection, Intersections};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

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
        let mut xs = Intersections::new(i1);
        xs.add(i2);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.items[0].t, 1.0);
        assert_eq!(xs.items[1].t, 2.0);
    }


    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let mut xs = Intersections::new(i1);
        xs.add(i2);

        let actual = xs.hit().unwrap();
        assert_eq!(actual, &Intersection::new(1.0, &sphere))
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let mut xs = Intersections::new(i1);
        xs.add(i2);

        let actual = xs.hit().unwrap();
        assert_eq!(actual, &Intersection::new(1.0, &sphere))
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(-2.0, &sphere);
        let mut xs = Intersections::new(i1);
        xs.add(i2);

        let actual = xs.hit();
        assert!(actual.is_none())
    }

    #[test]
    fn hit_is_always_lowest_non_negative_value() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let mut xs = Intersections::new(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4);

        let actual = xs.hit().unwrap();
        assert_eq!(actual, &Intersection::new(2.0, &sphere))
    }

    #[test]
    fn pre_computing_the_state_of_an_intersection() {
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = Computation::prepare(&i, &r).unwrap();
        assert_eq!(comps.t, 4.0);
        assert_eq!(*comps.object, shape);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_outside() {
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = Computation::prepare(&i, &r).unwrap();
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_inside() {
        let r = Ray::from_cords((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);

        let comps = Computation::prepare(&i, &r).unwrap();
        assert_eq!(comps.inside, true);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Tuple::vector(0.0, 0.0, -1.0));
    }
}
