use std::sync::atomic::{AtomicUsize, Ordering};

use crate::color::Color;
use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::{inverse, Matrix4x4, MATRIX_4X4_IDENTITY, mul_by_tuple, transpose};
use crate::ray::Ray;
use crate::tuple::Tuple;

static SPHERE_IDS: AtomicUsize = AtomicUsize::new(0);

// clone feels bad with the id .. but currently i don't now any better way to support multi threading
#[derive(Debug, Clone)]
pub struct Sphere {
    id: usize,
    transform: Matrix4x4,
    material: Material,
}

#[derive(Debug)]
pub enum SphereError {
    NormalWithVector,
    NormalFailedInverse,
}

impl Sphere {
    pub fn new() -> Sphere {
        let id = SPHERE_IDS.fetch_add(1, Ordering::SeqCst);
        Sphere { id, transform: MATRIX_4X4_IDENTITY, material: Material::default() }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersections> {
        let ray = ray.transform(inverse(self.transformation())?);

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

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);
        let mut xs = Intersections::new(i1);
        xs.add(i2);

        Some(xs)
    }

    pub fn transform(&mut self, t: Matrix4x4) {
        self.transform = t;
    }
    pub fn transformation(&self) -> Matrix4x4 {
        self.transform
    }

    pub fn normal_at(&self, point: Tuple) -> Result<Tuple, SphereError> {
        if point.is_vector() {
            return Err(SphereError::NormalWithVector);
        }
        let inverse_transformation = match inverse(self.transformation()) {
            Some(i) => i,
            None => return Err(SphereError::NormalFailedInverse)
        };
        let object_point = mul_by_tuple(inverse_transformation, point);
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = mul_by_tuple(transpose(inverse_transformation), object_normal);
        world_normal.w = 0.0;

        Ok(world_normal.normalize())
    }

    pub fn change_color(&mut self, color: Color) {
        self.material.change_color(color)
    }

    pub fn apply_mat(&mut self, mat: Material) {
        self.material = mat
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl std::cmp::PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::color::Color;
    use crate::material::Material;
    use crate::matrix::{MATRIX_4X4_IDENTITY, mul};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transformation::{rotation_z, scaling, translation};
    use crate::tuple::Tuple;

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, MATRIX_4X4_IDENTITY)
    }


    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();

        assert_eq!(4.0, xs.get(0).unwrap().t());
        assert_eq!(6.0, xs.get(1).unwrap().t());
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();

        assert_eq!(5.0, xs.get(0).unwrap().t());
        assert_eq!(5.0, xs.get(1).unwrap().t());
    }

    #[test]
    fn a_ray_missing_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert!(xs.is_none())
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();

        assert_eq!(-1.0, xs.get(0).unwrap().t());
        assert_eq!(1.0, xs.get(1).unwrap().t());
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let s = Sphere::new();
        let xs = s.intersect(&r).unwrap();

        assert_eq!(-6.0, xs.get(0).unwrap().t());
        assert_eq!(-4.0, xs.get(1).unwrap().t());
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(
            Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0),
        ).unwrap();
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray).unwrap();

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().object(), &sphere);
        assert_eq!(xs.get(1).unwrap().object(), &sphere)
    }

    #[test]
    fn changing_s_spheres_transformation() {
        let mut s = Sphere::new();
        let t = translation(2.0, 3.0, 4.0);
        s.transform(t);

        assert_eq!(s.transform, translation(2.0, 3.0, 4.0))
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let mut s = Sphere::new();
        s.transform(scaling(2.0, 2.0, 2.0));

        let xs = s.intersect(&r).unwrap();

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let mut s = Sphere::new();
        s.transform(scaling(5.0, 0.0, 0.0));

        let xs = s.intersect(&r);

        assert!(xs.is_none());
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let sphere = Sphere::new();
        let expected = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(sphere.normal_at(Tuple::point(1.0, 0.0, 0.0)).unwrap(), expected)
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let sphere = Sphere::new();
        let expected = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(sphere.normal_at(Tuple::point(0.0, 1.0, 0.0)).unwrap(), expected)
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let sphere = Sphere::new();
        let expected = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(sphere.normal_at(Tuple::point(0.0, 0.0, 5.0)).unwrap(), expected)
    }

    #[test]
    fn normal_on_sphere_at_point_on_non_axial_point() {
        let sphere = Sphere::new();
        let a = 3_f64.sqrt() / 3_f64;

        let actual = sphere.normal_at(Tuple::point(a, a, a)).unwrap();
        assert_eq!(actual, Tuple::vector(a, a, a))
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let sphere = Sphere::new();
        let a = 3_f64.sqrt() / 3_f64;
        let actual = sphere.normal_at(Tuple::point(a, a, a)).unwrap();

        assert_eq!(actual, actual.normalize())
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut sphere = Sphere::new();
        let translate = translation(0.0, 1.0, 0.0);
        sphere.transform(translate);

        let n = sphere.normal_at(Tuple::point(0.0, 1.70711, -0.70711)).unwrap();
        let expected = Tuple::vector(0.0, 0.70711, -0.70711);

        assert_eq!(n, expected)
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut sphere = Sphere::new();
        let m = mul(scaling(1.0, 0.5, 1.0), rotation_z(PI / 5.0));
        sphere.transform(m);

        let a = 2_f64.sqrt() / 2.0;
        let n = sphere.normal_at(Tuple::point(0.0, a, -a)).unwrap();

        assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254))
    }


    #[test]
    fn material_assignment() {
        let sphere_one = Sphere::new();
        let mut sphere_two = Sphere::new();
        let m = Material::new(Color::black(), 1.0, 1.0, 1.0, 100.0).unwrap();
        let m2 = Material::new(Color::black(), 1.0, 1.0, 1.0, 100.0).unwrap();
        sphere_two.apply_mat(m);

        assert_eq!(sphere_one.material, Material::default());
        assert_eq!(sphere_two.material, m2);
    }
}