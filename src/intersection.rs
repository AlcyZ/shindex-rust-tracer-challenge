use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering;

#[derive(Debug)]
pub(crate) struct Intersection<'a> {
    pub(crate) t: f64,
    pub(crate) object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub(crate) fn new(t: f64, object: &dyn Shape) -> Intersection {
        Intersection { t, object }
    }

    pub(crate) fn prepare_computation(&self, ray: Ray) -> Computation {
        let point = ray.position(self.t);
        let eye_v = -ray.direction;

        let mut normal_v = self.object.normal_at(point);
        let mut inside = false;
        if normal_v.dot(eye_v) < 0. {
            inside = true;
            normal_v = -normal_v;
        }
        let over_point = point + normal_v * EPSILON;

        Computation::new(
            self.t,
            self.object,
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        )
    }
}

#[derive(Debug)]
pub(crate) struct Computation<'a> {
    pub(crate) t: f64,
    pub(crate) object: &'a dyn Shape,
    pub(crate) point: Tuple,
    pub(crate) over_point: Tuple,
    pub(crate) eye_v: Tuple,
    pub(crate) normal_v: Tuple,
    pub(crate) inside: bool,
}

impl<'a> Computation<'a> {
    fn new(
        t: f64,
        object: &'a dyn Shape,
        point: Tuple,
        over_point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        inside: bool,
    ) -> Computation {
        Computation {
            t,
            object,
            point,
            over_point,
            eye_v,
            normal_v,
            inside,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Intersections<'a> {
    items: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub(crate) fn new() -> Intersections<'a> {
        Intersections { items: vec![] }
    }

    pub(crate) fn push(&mut self, intersection: Intersection<'a>) {
        self.items.push(intersection)
    }

    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Intersection> {
        match self.items.get(index) {
            Some(intersection) => Some(intersection),
            None => None,
        }
    }

    pub(crate) fn merge(&mut self, other: Intersections<'a>) {
        for item in other.items {
            self.items.push(item);
        }
    }

    pub(crate) fn sort(&mut self) {
        self.items.sort_by(|a, b| {
            if a.t < b.t {
                Ordering::Less
            } else if a.t == b.t {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        })
    }

    pub(crate) fn hit(&self) -> Option<&Intersection> {
        let mut result: Option<&Intersection> = None;

        for intersection in &self.items {
            if intersection.t > 0. {
                match result {
                    Some(i) => {
                        if intersection.t < i.t {
                            result = Some(intersection);
                        }
                    }
                    None => result = Some(intersection),
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::transformation::translation;
    use crate::math::EPSILON;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn test_intersection_encapsulate_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(s, i.object.into());
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        assert_eq!(2, xs.len());
        assert_eq!(1., xs.get(0).unwrap().t);
        assert_eq!(2., xs.get(1).unwrap().t);
    }

    #[test]
    fn test_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        let hit = xs.hit().unwrap();

        assert_eq!(1., hit.t);
    }

    #[test]
    fn test_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        let hit = xs.hit().unwrap();

        assert_eq!(1., hit.t);
    }

    #[test]
    fn test_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(-2., &s);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        assert!(xs.hit().is_none());
    }

    #[test]
    fn test_hit_is_always_lowest_negative_value() {
        let s = Sphere::new();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);
        xs.push(i3);
        xs.push(i4);

        let hit = xs.hit().unwrap();

        assert_eq!(2., hit.t);
    }

    #[test]
    fn test_precomputing_state_of_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();
        let i = Intersection::new(4., &s);

        let computation = i.prepare_computation(r);

        assert!(std::ptr::eq(i.object, computation.object));
        assert_eq!(computation.point, Tuple::point(0., 0., -1.));
        assert_eq!(computation.eye_v, Tuple::direction(0., 0., -1.));
        assert_eq!(computation.normal_v, Tuple::direction(0., 0., -1.));
    }

    #[test]
    fn test_hit_when_intersection_occurs_on_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();
        let i = Intersection::new(4., &s);

        let comps = i.prepare_computation(r);

        assert!(!comps.inside);
    }

    #[test]
    fn test_hit_when_intersection_occurs_on_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();
        let i = Intersection::new(1., &s);

        let comps = i.prepare_computation(r);

        assert!(comps.inside);
        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eye_v, Tuple::direction(0., 0., -1.));
        assert_eq!(comps.normal_v, Tuple::direction(0., 0., -1.));
    }

    #[test]
    fn test_hit_should_offset_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let mut shape = Sphere::new();
        shape.mut_props().set_transform(translation(0., 0., 1.));

        let i = Intersection::new(5., &shape);
        let comps = i.prepare_computation(r);

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z)
    }
}
