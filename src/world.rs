use crate::color::Color;
use crate::intersection::{Computation, Intersections};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

pub(crate) struct World {
    pub(crate) light: Option<PointLight>,
    pub(crate) objects: Vec<Box<dyn Shape>>,
}

impl World {
    pub(crate) fn new() -> World {
        World {
            light: None,
            objects: vec![],
        }
    }

    pub(crate) fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match xs.hit() {
            Some(hit) => {
                let comp = hit.prepare_computation(ray);

                self.shade_hit(&comp)
            }
            None => Color::new(0., 0., 0.),
        }
    }

    fn is_shadowed(&self, point: Tuple) -> bool {
        let light_position = match self.light {
            Some(light) => light.position,
            None => return false,
        };

        let direction_v = light_position - point;
        let distance = direction_v.magnitude();
        let direction = direction_v.normalize();

        let xs = self.intersect(Ray::new(point, direction));

        match xs.hit() {
            Some(hit) => hit.t < distance,
            None => false,
        }
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new();

        for object in &self.objects {
            if let Some(i) = object.intersect(ray) {
                xs.merge(i);
            }
        }
        xs.sort();

        xs
    }

    fn shade_hit(&self, computation: &Computation) -> Color {
        let is_shadowed = self.is_shadowed(computation.over_point);

        // Todo: Fix unwrap
        computation.object.get_props().get_material().lighting(
            self.light.unwrap(),
            computation.over_point,
            computation.eye_v,
            computation.normal_v,
            is_shadowed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::intersection::Intersection;
    use crate::math::transformation::{scaling, translation};
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    fn default_world() -> World {
        let point_light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::new();
        s1.mut_props().set_material_color(Color::new(0.8, 1., 0.6));
        s1.mut_props().set_material_diffuse(0.7);
        s1.mut_props().set_material_specular(0.2);

        let mut s2 = Sphere::new();
        s2.mut_props().set_transform(scaling(0.5, 0.5, 0.5));

        let mut w = World::new();
        w.light = Some(point_light);
        w.objects.push(Box::new(s1));
        w.objects.push(Box::new(s2));

        w
    }

    #[test]
    fn test_creating_world() {
        let w = World::new();

        assert!(w.light.is_none());
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_intersect_world_with_ray() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 0.0, 1.));

        let xs = w.intersect(r);

        assert_eq!(4, xs.len());
        assert_eq!(4., xs.get(0).unwrap().t);
        assert_eq!(4.5, xs.get(1).unwrap().t);
        assert_eq!(5.5, xs.get(2).unwrap().t);
        assert_eq!(6., xs.get(3).unwrap().t);
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 0.0, 1.));
        let shape = w.objects.first().unwrap();
        let i = Intersection::new(4., shape.as_ref());

        let comps = i.prepare_computation(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_inside() {
        let mut w = default_world();
        w.light = Some(PointLight::new(
            Tuple::point(0., 0.25, 0.),
            Color::new(1., 1., 1.),
        ));

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0.0, 0.0, 1.));
        let shape = w.objects.last().unwrap();
        let i = Intersection::new(0.5, shape.as_ref());

        let comps = i.prepare_computation(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_shade_hit_with_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Tuple::point(0., 0., -10.),
            Color::new(1., 1., 1.),
        ));

        let s1 = Sphere::new();
        w.objects.push(Box::new(s1));

        let mut s2 = Sphere::new();
        s2.mut_props().set_transform(translation(0., 0., 10.));
        w.objects.push(Box::new(s2));

        let s2_ref = w.objects.last().unwrap();

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::direction(0., 0., 1.));
        let i = Intersection::new(4., s2_ref.as_ref());

        let comps = i.prepare_computation(r);
        let c = w.shade_hit(&comps);

        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn test_color_when_ray_miss() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 1.0, 0.));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 0.0, 1.));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        let point_light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::new();
        s1.mut_props().set_material_color(Color::new(0.8, 1., 0.6));
        s1.mut_props().set_material_diffuse(0.7);
        s1.mut_props().set_material_specular(0.2);
        s1.mut_props().set_material_ambient(1.);

        let mut s2 = Sphere::new();
        s2.mut_props().set_transform(scaling(0.5, 0.5, 0.5));
        s2.mut_props().set_material_ambient(1.);

        let mut w = World::new();
        w.light = Some(point_light);
        w.objects.push(Box::new(s1));
        w.objects.push(Box::new(s2));

        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::direction(0., 0., -1.));
        let c = w.color_at(ray);

        assert_eq!(c, s2.get_props().get_material().get_color());
    }

    #[test]
    fn test_no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = default_world();
        let p = Tuple::point(0., 10., 0.);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn test_shadow_when_object_is_between_point_and_light() {
        let w = default_world();
        let p = Tuple::point(10., -10., 10.);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn test_no_shadow_when_object_is_behind_light() {
        let w = default_world();
        let p = Tuple::point(-20., 20., 20.);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn test_no_shadow_when_object_is_behind_point() {
        let w = default_world();
        let p = Tuple::point(-2., 2., 2.);

        assert!(!w.is_shadowed(p));
    }
}
