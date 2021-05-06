use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Intersection<'a> {
    pub(crate) id: Uuid,
    pub(crate) t: f64,
    pub(crate) object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub(crate) fn new(t: f64, object: &dyn Shape) -> Intersection {
        Intersection {
            id: Uuid::new_v4(),
            t,
            object,
        }
    }

    pub(crate) fn prepare_computation(&self, ray: Ray, xs: &Intersections) -> Computation {
        let point = ray.position(self.t);
        let eye_v = -ray.direction;

        let mut normal_v = self.object.normal_at(point);
        let mut inside = false;
        if normal_v.dot(eye_v) < 0. {
            inside = true;
            normal_v = -normal_v;
        }
        let reflect_v = ray.direction.reflect(normal_v);
        let over_point = point + normal_v * EPSILON;
        let under_point = point - normal_v * EPSILON;

        let mut containers: Vec<&dyn Shape> = vec![];

        let mut n1 = 0.0;
        let mut n2 = 0.0;

        for i in &xs.items {
            // If the intersection is the hit , set n1 to the refractive index of the last object
            // in the containers list. If that list is empty, then there is no containing object,
            // and n1 should be set to 1.0
            if i == self {
                if containers.is_empty() {
                    n1 = 1.;
                } else {
                    n1 = containers
                        .last()
                        .unwrap()
                        .get_props()
                        .get_material()
                        .get_refractive_index();
                }
            }

            // if the intersection’s object is already in the containers list, then this inter-
            // section must be exiting the object. Remove the object from the containers
            // list in this case. Otherwise, the intersection is entering the object, and
            // the object should be added to the end of the list.
            match containers
                .iter()
                .position(|shape| shape.get_id() == i.object.get_id())
            {
                Some(index) => {
                    containers.remove(index);
                }
                None => containers.push(i.object),
            }

            // If the intersection is the hit , set n2 to the refractive index of the last object
            // in the containers list. If that list is empty, then again, there is no containing
            // object and n2 should be set to 1.0
            if i == self {
                if containers.is_empty() {
                    n2 = 1.;
                } else {
                    n2 = containers
                        .last()
                        .unwrap()
                        .get_props()
                        .get_material()
                        .get_refractive_index();
                }
                // If the intersection is the hit , terminate the loop here.
                break;
            }
        }

        Computation::new(
            self.t,
            self.object,
            point,
            over_point,
            under_point,
            eye_v,
            normal_v,
            reflect_v,
            inside,
            n1,
            n2,
        )
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub(crate) struct Computation<'a> {
    pub(crate) t: f64,
    pub(crate) object: &'a dyn Shape,
    pub(crate) point: Tuple,
    pub(crate) over_point: Tuple,
    pub(crate) under_point: Tuple,
    pub(crate) eye_v: Tuple,
    pub(crate) normal_v: Tuple,
    pub(crate) reflect_v: Tuple,
    pub(crate) inside: bool,
    pub(crate) n1: f64,
    pub(crate) n2: f64,
}

impl<'a> Computation<'a> {
    fn new(
        t: f64,
        object: &'a dyn Shape,
        point: Tuple,
        over_point: Tuple,
        under_point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        reflect_v: Tuple,
        inside: bool,
        n1: f64,
        n2: f64,
    ) -> Computation {
        Computation {
            t,
            object,
            point,
            over_point,
            under_point,
            eye_v,
            normal_v,
            reflect_v,
            inside,
            n1,
            n2,
        }
    }

    pub(crate) fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eye_v.dot(self.normal_v);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1. - cos.powi(2));

            if sin2_t > 1. {
                return 1.;
            }

            // compute cosine of theta_t using trig identity
            let cos_t = (1. - sin2_t).sqrt();
            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t;
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1. - r0) * (1. - cos).powi(5)
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

    pub(crate) fn first(&self) -> Option<&Intersection> {
        self.items.first()
    }

    pub(crate) fn last(&self) -> Option<&Intersection> {
        self.items.last()
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
    use crate::math::transformation::{scaling, translation};
    use crate::math::{f64_eq, EPSILON};
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn test_intersection_encapsulate_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(s.get_id(), i.object.get_id());
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

        let xs = Intersections::new();
        let computation = i.prepare_computation(r, &xs);

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

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);

        assert!(!comps.inside);
    }

    #[test]
    fn test_hit_when_intersection_occurs_on_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 0., 1.));
        let s = Sphere::new();
        let i = Intersection::new(1., &s);

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);

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
        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);

        assert!(comps.over_point.z < -EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z)
    }

    #[test]
    fn test_precomputing_the_reflection_vector() {
        let shape = Plane::new();
        let r = Ray::new(
            Tuple::point(0., 1., -1.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let i = Intersection::new(2f64.sqrt(), &shape);
        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);

        assert_eq!(
            Tuple::direction(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.),
            comps.reflect_v
        );
    }

    #[test]
    fn test_finding_n1_and_n2_at_various_intersections() {
        let mut a = Sphere::glass();
        a.mut_props().set_transform(scaling(2., 2., 2.));

        let mut b = Sphere::glass();
        b.mut_props().set_transform(translation(0., 0., -0.25));
        b.mut_props().set_material_refractive_index(2.);

        let mut c = Sphere::glass();
        c.mut_props().set_transform(translation(0., 0., 0.25));
        c.mut_props().set_material_refractive_index(2.5);

        let r = Ray::new(Tuple::point(0., 0., -4.), Tuple::direction(0., 0., 1.));

        let mut xs = Intersections::new();
        xs.items.push(Intersection::new(2., &a));
        xs.items.push(Intersection::new(2.75, &b));
        xs.items.push(Intersection::new(3.25, &c));
        xs.items.push(Intersection::new(4.75, &b));
        xs.items.push(Intersection::new(5.25, &c));
        xs.items.push(Intersection::new(6., &a));

        let comps = xs.get(0).unwrap().prepare_computation(r, &xs);
        let expected = (1.0, 1.5);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);

        let comps = xs.get(1).unwrap().prepare_computation(r, &xs);
        let expected = (1.5, 2.0);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);

        let comps = xs.get(2).unwrap().prepare_computation(r, &xs);
        let expected = (2.0, 2.5);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);

        let comps = xs.get(3).unwrap().prepare_computation(r, &xs);
        let expected = (2.5, 2.5);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);

        let comps = xs.get(4).unwrap().prepare_computation(r, &xs);
        let expected = (2.5, 1.5);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);

        let comps = xs.get(5).unwrap().prepare_computation(r, &xs);
        let expected = (1.5, 1.0);
        let actual = (comps.n1, comps.n2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let mut shape = Sphere::glass();
        shape.mut_props().set_transform(translation(0., 0., 1.));

        let i = Intersection::new(5., &shape);
        let mut xs = Intersections::new();
        xs.items.push(i);

        // because we dont care about n1 + n2, we can bypass the calculation by creating another
        // intersection instance
        let i = Intersection::new(5., &shape);
        let comps = i.prepare_computation(r, &xs);

        assert!(comps.under_point.z > EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn test_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass();

        let r = Ray::new(
            Tuple::point(0., 0., 2f64.sqrt() / 2.),
            Tuple::direction(0., 1., 0.),
        );

        let mut xs = Intersections::new();
        xs.push(Intersection::new(-2f64.sqrt() / 2., &shape));
        xs.push(Intersection::new(2f64.sqrt() / 2., &shape));

        let comps = xs.last().unwrap().prepare_computation(r, &xs);
        let reflectance = comps.schlick();

        assert_eq!(1., reflectance);
    }

    #[test]
    fn test_schlick_approximation_with_perpendicular_viewing_angle() {
        let shape = Sphere::glass();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 1., 0.));

        let mut xs = Intersections::new();
        xs.push(Intersection::new(-1., &shape));
        xs.push(Intersection::new(1., &shape));

        let comps = xs.last().unwrap().prepare_computation(r, &xs);
        let reflectance = comps.schlick();

        assert!(f64_eq(0.04, reflectance));
    }

    #[test]
    fn test_schlick_approximation_with_small_angle_and_n2_greater_n1() {
        let shape = Sphere::glass();
        let r = Ray::new(Tuple::point(0., 0.99, -2.), Tuple::direction(0., 0., 1.));

        let mut xs = Intersections::new();
        xs.push(Intersection::new(1.8589, &shape));

        let comps = xs.first().unwrap().prepare_computation(r, &xs);
        let reflectance = comps.schlick();

        assert!(f64_eq(0.48873, reflectance));
    }
}
