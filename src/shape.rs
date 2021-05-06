use crate::color::Color;
use crate::intersection::Intersections;
use crate::material::Material;
use crate::math::matrix::M4;
use crate::pattern::Pattern;
use crate::ray::Ray;
use crate::tuple::Tuple;
use std::fmt::Debug;
use uuid::Uuid;

pub(crate) trait Shape: Debug + Sync + Send {
    fn get_props(&self) -> &ShapeProps;

    fn mut_props(&mut self) -> &mut ShapeProps;

    fn local_normal_at(&self, point: Tuple) -> Tuple;

    fn local_intersect(&self, ray: Ray) -> Option<Intersections>;

    fn get_id(&self) -> Uuid {
        self.get_props().id
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let inverse_transform = self.get_props().get_transform().inverse().unwrap();
        let local_point = inverse_transform * point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = inverse_transform.transpose() * local_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }

    fn intersect(&self, ray: Ray) -> Option<Intersections> {
        let local_ray = ray.transform(self.get_props().get_transform().inverse().unwrap());

        self.local_intersect(local_ray)
    }
}

#[derive(Debug)]
pub(crate) struct ShapeProps {
    id: Uuid,
    transform: M4,
    material: Material,
}

impl ShapeProps {
    pub(crate) fn default() -> ShapeProps {
        ShapeProps {
            id: Uuid::new_v4(),
            transform: M4::identity(),
            material: Material::new(),
        }
    }

    pub(crate) fn get_transform(&self) -> M4 {
        self.transform
    }

    pub(crate) fn set_transform(&mut self, new: M4) {
        self.transform = new
    }

    pub(crate) fn get_material(&self) -> &Material {
        &self.material
    }

    pub(crate) fn set_material(&mut self, new: Material) {
        self.material = new
    }

    pub(crate) fn set_material_color(&mut self, new: Color) {
        self.material.set_color(new)
    }

    pub(crate) fn set_material_ambient(&mut self, new: f64) {
        self.material.set_ambient(new);
    }

    pub(crate) fn set_material_diffuse(&mut self, new: f64) {
        self.material.set_diffuse(new);
    }

    pub(crate) fn set_material_specular(&mut self, new: f64) {
        self.material.set_specular(new);
    }

    pub(crate) fn set_material_reflective(&mut self, new: f64) {
        self.material.set_reflective(new);
    }

    pub(crate) fn _set_material_shininess(&mut self, new: f64) {
        self.material._set_shininess(new);
    }

    pub(crate) fn set_material_transparency(&mut self, new: f64) {
        self.material.set_transparency(new);
    }
    pub(crate) fn set_material_refractive_index(&mut self, new: f64) {
        self.material.set_refractive_index(new);
    }

    pub(crate) fn set_pattern(&mut self, new: Box<dyn Pattern>) {
        self.material.set_pattern(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::transformation::{rotation_z, scaling, translation};
    use std::f64::consts::PI;

    #[derive(Debug)]
    struct TestShape {
        props: ShapeProps,
    }

    impl TestShape {
        fn new() -> TestShape {
            TestShape {
                props: ShapeProps::default(),
            }
        }

        fn to_local_ray(&self, ray: Ray) -> Ray {
            ray.transform(self.props.transform.inverse().unwrap())
        }
    }

    impl PartialEq for TestShape {
        fn eq(&self, other: &Self) -> bool {
            self.get_id() == other.get_id()
        }
    }

    impl Shape for TestShape {
        fn get_props(&self) -> &ShapeProps {
            &self.props
        }

        fn mut_props(&mut self) -> &mut ShapeProps {
            &mut self.props
        }

        fn local_normal_at(&self, point: Tuple) -> Tuple {
            Tuple::direction(point.x, point.y, point.z)
        }

        fn local_intersect(&self, _ray: Ray) -> Option<Intersections> {
            None
        }
    }

    #[test]
    fn test_shape_default_transform() {
        let s = TestShape::new();

        assert_eq!(s.props.transform, M4::identity());
    }

    #[test]
    fn test_assign_transformation() {
        let mut s = TestShape::new();
        s.props.set_transform(translation(2., 3., 4.));

        assert_eq!(s.props.transform, translation(2., 3., 4.));
    }

    #[test]
    fn test_shape_default_material() {
        let s = TestShape::new();

        assert_eq!(s.props.material, Material::new());
    }

    #[test]
    fn test_shape_assigning_material() {
        let mut s = TestShape::new();
        let mut other = Material::new();
        other.set_ambient(1.);
        s.props.set_material(other);

        let mut e = Material::new();
        e.set_ambient(1.);

        assert_eq!(s.props.material, e);
    }

    #[test]
    fn test_intersect_scaled_shape_with_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let mut s = TestShape::new();

        s.props.set_transform(scaling(2., 2., 2.));
        let r2 = s.to_local_ray(r);

        assert_eq!(r2.origin, Tuple::point(0., 0., -2.5));
        assert_eq!(r2.direction, Tuple::direction(0., 0., 0.5));
    }

    #[test]
    fn test_intersect_translated_shape_with_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::direction(0., 0., 1.));
        let mut s = TestShape::new();

        s.props.set_transform(translation(5., 0., 0.));
        let r2 = s.to_local_ray(r);

        assert_eq!(r2.origin, Tuple::point(-5., 0., -5.));
        assert_eq!(r2.direction, Tuple::direction(0., 0., 1.));
    }

    #[test]
    fn test_compute_normal_on_translated_shape() {
        let mut s = TestShape::new();
        s.props.set_transform(translation(0., 1., 0.));

        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));
        assert_eq!(n, Tuple::direction(0., 0.70711, -0.70711));
    }

    #[test]
    fn test_compute_normal_on_asd_shape() {
        let mut s = TestShape::new();
        let m = scaling(1., 0.5, 1.) * rotation_z(PI / 5.);
        s.props.set_transform(m);

        let n = s.normal_at(Tuple::point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.));
        assert_eq!(n, Tuple::direction(0., 0.97014, -0.24254));
    }
}
