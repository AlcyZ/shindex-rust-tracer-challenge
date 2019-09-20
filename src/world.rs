use crate::color::Color;
use crate::intersection::{Computation, Intersections};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformation::scaling;

#[derive(Debug)]
pub struct World {
    light: Option<PointLight>,
    objects: Option<Vec<Sphere>>,
}

impl World {
    pub fn new() -> World {
        World { light: None, objects: None }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersections> {
        if self.objects.is_none() {
            return None;
        }

        let mut xs = vec![];
        for object in self.objects.as_ref().unwrap() {
            if let Some(intersection) = object.intersect((&ray)) {
                let [first, second] = intersection;

                xs.push(first);
                xs.push(second);
            }
        }

        if xs.len() == 0 {
            return None;
        }
        xs.sort_by(|i, b| i.t().partial_cmp(&b.t()).unwrap());

        Some(Intersections::from_intersections(xs))
    }

    pub fn shade_hit(&self, computation: &Computation) -> Color {
        computation.material().lighting(
            self.light.unwrap(),
            computation.point,
            computation.eye_v,
            computation.normal_v,
        )
    }


    pub fn color_at(&self, ray: &Ray) -> Color {
        match self._col(&ray) {
            Some(c) => c,
            None => Color::black()
        }
    }

    fn _col(&self, ray: &Ray) -> Option<Color> {
        let xs = self.intersect(ray)?;
        let hit = xs.hit()?;
        let computation = Computation::prepare(&hit, ray)?;

        Some(self.shade_hit(&computation))
    }
}

pub fn test_default_world() -> World {
    let light = PointLight::from_cords(-10.0, -10.0, -10.0, Color::white());
    let mut s1 = Sphere::new();
    s1.change_color(Color::new(0.8, 1.0, 0.6));
    s1.change_diffuse(0.7);
    s1.change_specular(0.2);

    let mut s2 = Sphere::new();
    s2.transform(scaling(0.5, 0.5, 0.5));

    World { light: Some(light), objects: Some(vec![s1, s2]) }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::color::Color;
    use crate::intersection::{Computation, Intersection};
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transformation::scaling;
    use crate::world::{test_default_world, World};

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.light.is_none());
        assert!(w.objects.is_none())
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = test_default_world();
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        let xs = w.intersect(&r).unwrap();

        assert_eq!(xs.count(), 4);
        assert_eq!(xs.get(0).unwrap().t(), 4.0);
        assert_eq!(xs.get(1).unwrap().t(), 4.5);
        assert_eq!(xs.get(2).unwrap().t(), 5.5);
        assert_eq!(xs.get(3).unwrap().t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = test_default_world();
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        let shapes = w.objects.as_ref().unwrap();
        let shape = shapes.first().unwrap();

        let i = Intersection::new(4.0, shape);
        let comps = Computation::prepare(&i, &r).unwrap();
        let color = w.shade_hit(&comps);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = test_default_world();
        w.light = Some(PointLight::from_cords(0.0, 0.25, 0.0, Color::white()));
        let r = Ray::from_cords((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let shape = w.objects.as_ref().unwrap().last().unwrap();
        let i = Intersection::new(0.5, &shape);
        let comps = Computation::prepare(&i, &r).unwrap();
        let color = w.shade_hit(&comps);

        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498))
    }

    #[test]
    fn color_then_ray_misses() {
        let w = test_default_world();
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 1.0, 0.0));
        let color = w.color_at(&r);

        assert_eq!(color, Color::black())
    }

    #[test]
    fn color_then_ray_hits() {
        let w = test_default_world();
        let r = Ray::from_cords((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let color = w.color_at(&r);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = test_default_world();
        let mut outer = w.objects.as_mut().unwrap().first().unwrap().clone();
        outer.change_ambient(1.0);
        let mut inner = w.objects.as_mut().unwrap().last().unwrap().clone();
        inner.change_color(Color::new(0.1, 0.1, 0.1));

        let r = Ray::from_cords((0.0, 0.0, 0.75), (0.0, 0.0, -1.0));
        let color = w.color_at(&r);

        assert_eq!(color, inner.material().color())
    }
}
