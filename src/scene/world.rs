use crate::primitives::shape::Shape;
use crate::scene::shading::color::Color;
use crate::scene::shading::light::PointLight;
use crate::scene::tracing::intersection::{Computation, Intersections};
use crate::scene::tracing::ray::Ray;

use crate::math::tuple::Tuple;

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

    pub(crate) fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        let xs = self.intersect(ray);
        match xs.hit() {
            Some(hit) => {
                let comp = hit.prepare_computation(ray, &xs);

                self.shade_hit(&comp, remaining)
            }
            None => Color::new(0., 0., 0.),
        }
    }

    fn reflected_color(&self, computation: &Computation, remaining: usize) -> Color {
        if remaining <= 0 {
            return Color::new(0., 0., 0.);
        }

        let reflective = computation
            .object
            .get_props()
            .get_material()
            .get_reflective();
        if reflective == 0. {
            return Color::new(0., 0., 0.);
        }
        let reflect_ray = Ray::new(computation.over_point, computation.reflect_v);
        let color = self.color_at(reflect_ray, remaining - 1);

        color * reflective
    }

    fn refracted_color(&self, computation: &Computation, remaining: usize) -> Color {
        if remaining <= 0 {
            return Color::black();
        }

        if computation
            .object
            .get_props()
            .get_material()
            .get_transparency()
            == 0.
        {
            Color::black()
        } else {
            let n_ratio = computation.n1 / computation.n2;
            let cos_i = computation.eye_v.dot(computation.normal_v);
            let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));

            if sin2_t > 1. {
                return Color::black();
            }

            let cos_t = (1. - sin2_t).sqrt();
            let direction =
                computation.normal_v * (n_ratio * cos_i - cos_t) - computation.eye_v * n_ratio;

            let refract_ray = Ray::new(computation.under_point, direction);

            self.color_at(refract_ray, remaining - 1)
                * computation
                    .object
                    .get_props()
                    .get_material()
                    .get_transparency()
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

    fn shade_hit(&self, computation: &Computation, remaining: usize) -> Color {
        let is_shadowed = self.is_shadowed(computation.over_point);

        // Todo: Fix unwrap
        let surface = computation.object.get_props().get_material().lighting(
            computation.object,
            self.light.unwrap(),
            computation.over_point,
            computation.eye_v,
            computation.normal_v,
            is_shadowed,
        );
        let reflected = self.reflected_color(&computation, remaining);
        let refracted = self.refracted_color(&computation, remaining);

        let m = computation.object.get_props().get_material();

        if m.get_reflective() > 0. && m.get_transparency() > 0. {
            let reflectance = computation.schlick();

            return surface + reflected * reflectance + refracted * (1. - reflectance);
        }

        surface + reflected + refracted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::transformation::{scaling, translation};
    use crate::math::tuple::Tuple;
    use crate::pattern::{Pattern, PatternProps};
    use crate::primitives::plane::Plane;
    use crate::primitives::sphere::Sphere;
    use crate::scene::shading::color::Color;
    use crate::scene::tracing::intersection::Intersection;
    use crate::scene::tracing::ray::Ray;

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

    #[derive(Copy, Clone, Debug)]
    struct TestPattern {
        props: PatternProps,
    }

    impl Pattern for TestPattern {
        fn pattern_at(&self, point: Tuple) -> Color {
            Color::new(point.x, point.y, point.z)
        }

        fn get_props(&self) -> &PatternProps {
            &self.props
        }

        fn mut_props(&mut self) -> &mut PatternProps {
            &mut self.props
        }
    }

    fn test_pattern() -> TestPattern {
        TestPattern {
            props: PatternProps::default(),
        }
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

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let c = w.shade_hit(&comps, 0);

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

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let c = w.shade_hit(&comps, 4);

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

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let c = w.shade_hit(&comps, 4);

        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn test_color_when_ray_miss() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 1.0, 0.));

        let c = w.color_at(r, 4);

        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0.0, 0.0, 1.));

        let c = w.color_at(r, 4);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        let mut w = default_world();
        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_ambient(1.);
        w.objects
            .last_mut()
            .unwrap()
            .mut_props()
            .set_material_ambient(1.);

        let e = w
            .objects
            .last()
            .unwrap()
            .get_props()
            .get_material()
            .get_color();

        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::direction(0., 0., -1.));
        let c = w.color_at(ray, 4);

        assert_eq!(c, e);
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

    #[test]
    fn test_reflected_color_for_non_reflective_material() {
        let point_light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::new();
        s1.mut_props().set_material_color(Color::new(0.8, 1., 0.6));
        s1.mut_props().set_material_diffuse(0.7);
        s1.mut_props().set_material_specular(0.2);

        let mut s2 = Sphere::new();
        s2.mut_props().set_transform(scaling(0.5, 0.5, 0.5));
        s2.mut_props().set_material_ambient(1.);

        let mut w = World::new();
        w.light = Some(point_light);
        w.objects.push(Box::new(s1));
        w.objects.push(Box::new(s2));

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 0., 1.));
        let shape = w.objects.last().unwrap();
        let i = Intersection::new(1., shape.as_ref());

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let color = w.reflected_color(&comps, 0);

        assert_eq!(Color::new(0., 0., 0.), color);
    }

    #[test]
    fn test_reflected_color_for_reflective_material() {
        let mut w = default_world();

        let mut shape = Plane::new();
        shape.mut_props().set_material_reflective(0.5);
        shape.mut_props().set_transform(translation(0., -1., 0.));
        w.objects.push(Box::new(shape));

        let shape = w.objects.last().unwrap().as_ref();

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let i = Intersection::new(2f64.sqrt(), shape);

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(color, Color::new(0.190332, 0.23791, 0.142749));
    }

    #[test]
    fn test_shade_hit_for_reflective_material() {
        let mut w = default_world();

        let mut shape = Plane::new();
        shape.mut_props().set_material_reflective(0.5);
        shape.mut_props().set_transform(translation(0., -1., 0.));
        w.objects.push(Box::new(shape));

        let shape = w.objects.last().unwrap().as_ref();

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let i = Intersection::new(2f64.sqrt(), shape);

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let color = w.shade_hit(&comps, 4);

        assert_eq!(color, Color::new(0.876757, 0.9243403, 0.829174));
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Tuple::point(0., 0., 0.),
            Color::new(1., 1., 1.),
        ));

        let mut lower = Plane::new();
        lower.mut_props().set_material_reflective(1.);
        lower.mut_props().set_transform(translation(0., -1., 0.));

        let mut upper = Plane::new();
        upper.mut_props().set_material_reflective(1.);
        upper.mut_props().set_transform(translation(0., 1., 0.));

        w.objects.push(Box::new(lower));
        w.objects.push(Box::new(upper));

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::direction(0., 1., 0.));
        w.color_at(r, 4);

        assert!(true)
    }

    #[test]
    fn test_reflected_color_at_maximum_recursive_depth() {
        let mut w = default_world();

        let mut shape = Plane::new();
        shape.mut_props().set_material_reflective(0.5);
        shape.mut_props().set_transform(translation(0., -1., 0.));
        w.objects.push(Box::new(shape));

        let shape = w.objects.last().unwrap().as_ref();

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let i = Intersection::new(2f64.sqrt(), shape);

        let xs = Intersections::new();
        let comps = i.prepare_computation(r, &xs);
        let color = w.reflected_color(&comps, 0);

        assert_eq!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_refracted_color_with_opaque_surface() {
        let w = default_world();
        let shape = w.objects.first().unwrap().as_ref();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));

        let mut xs = Intersections::new();
        xs.push(Intersection::new(4., shape));
        xs.push(Intersection::new(6., shape));

        let comps = xs.first().unwrap().prepare_computation(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Color::black())
    }

    #[test]
    fn test_refracted_color_at_maximum_recursive_depth() {
        let mut w = default_world();
        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_transparency(1.);
        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_refractive_index(1.5);

        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let mut xs = Intersections::new();
        xs.push(Intersection::new(4., w.objects.first().unwrap().as_ref()));
        xs.push(Intersection::new(6., w.objects.first().unwrap().as_ref()));

        let comps = xs.first().unwrap().prepare_computation(r, &xs);
        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, Color::black())
    }

    #[test]
    fn test_refracted_color_under_total_internal_reflection() {
        let mut w = default_world();

        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_transparency(1.);
        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_refractive_index(1.5);

        let r = Ray::new(
            Tuple::point(0., 0., 2f64.sqrt() / 2.),
            Tuple::direction(0., 1., 0.),
        );
        let mut xs = Intersections::new();
        xs.push(Intersection::new(
            -2f64.sqrt() / 2.,
            w.objects.first().unwrap().as_ref(),
        ));
        xs.push(Intersection::new(
            2f64.sqrt() / 2.,
            w.objects.first().unwrap().as_ref(),
        ));

        let comps = xs.last().unwrap().prepare_computation(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Color::black())
    }

    #[test]
    fn test_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();

        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_material_ambient(1.);
        w.objects
            .first_mut()
            .unwrap()
            .mut_props()
            .set_pattern(Box::new(test_pattern()));

        w.objects
            .last_mut()
            .unwrap()
            .mut_props()
            .set_material_transparency(1.);
        w.objects
            .last_mut()
            .unwrap()
            .mut_props()
            .set_material_refractive_index(1.5);

        let r = Ray::new(Tuple::point(0., 0., 0.1), Tuple::direction(0., 1., 0.));

        let mut xs = Intersections::new();
        xs.push(Intersection::new(
            -0.9899,
            w.objects.first().unwrap().as_ref(),
        ));
        xs.push(Intersection::new(
            -0.4899,
            w.objects.last().unwrap().as_ref(),
        ));
        xs.push(Intersection::new(
            0.4899,
            w.objects.last().unwrap().as_ref(),
        ));
        xs.push(Intersection::new(
            0.9899,
            w.objects.first().unwrap().as_ref(),
        ));

        let comps = xs.get(2).unwrap().prepare_computation(r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(c, Color::new(0., 0.998874, 0.04721));
    }

    #[test]
    fn test_shade_hit_with_transparent_material() {
        let mut w = default_world();

        let mut floor = Plane::new();
        floor.mut_props().set_transform(translation(0., -1., 0.));
        floor.mut_props().set_material_transparency(0.5);
        floor.mut_props().set_material_refractive_index(1.5);
        w.objects.push(Box::new(floor));

        let mut ball = Sphere::new();
        ball.mut_props().set_material_color(Color::new(1., 0., 0.));
        ball.mut_props().set_material_ambient(0.5);
        ball.mut_props().set_transform(translation(0., -3.5, -0.5));
        w.objects.push(Box::new(ball));

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let mut xs = Intersections::new();
        xs.push(Intersection::new(
            2f64.sqrt(),
            w.objects.get(2).unwrap().as_ref(),
        ));

        let comps = xs.first().unwrap().prepare_computation(r, &xs);

        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn test_shade_hit_with_reflective_and_transparent_material() {
        let mut w = default_world();

        let mut floor = Plane::new();
        floor.mut_props().set_transform(translation(0., -1., 0.));
        floor.mut_props().set_material_reflective(0.5);
        floor.mut_props().set_material_transparency(0.5);
        floor.mut_props().set_material_refractive_index(1.5);
        w.objects.push(Box::new(floor));

        let mut ball = Sphere::new();
        ball.mut_props().set_material_color(Color::new(1., 0., 0.));
        ball.mut_props().set_material_ambient(0.5);
        ball.mut_props().set_transform(translation(0., -3.5, -0.5));
        w.objects.push(Box::new(ball));

        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::direction(0., -2f64.sqrt() / 2., 2f64.sqrt() / 2.),
        );
        let mut xs = Intersections::new();
        xs.push(Intersection::new(
            2f64.sqrt(),
            w.objects.get(2).unwrap().as_ref(),
        ));

        let comps = xs.first().unwrap().prepare_computation(r, &xs);

        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
