use crate::ray::Ray;
use crate::matrix::{Matrix4x4, MATRIX_4X4_IDENTITY};
use std::sync::atomic::{AtomicUsize, Ordering};

static SPHERE_IDS: AtomicUsize = AtomicUsize::new(0);

// clone feels bad with the id .. but currently i don't now any better way to support multi threading
#[derive(Debug, Clone)]
pub struct Sphere {
    id: usize,
    transform: Matrix4x4,
}

impl Sphere {
    pub fn new() -> Sphere {
        let id = SPHERE_IDS.fetch_add(1, Ordering::SeqCst);
        Sphere { id, transform: MATRIX_4X4_IDENTITY }
    }

    pub fn transform(&mut self, t: Matrix4x4) {
        self.transform = t;
    }
    pub fn transformation(&self) -> Matrix4x4 {
        self.transform
    }
}

impl std::cmp::PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::MATRIX_4X4_IDENTITY;
    use crate::sphere::Sphere;
    use crate::transformation::{translation, scaling};
    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::intersection::intersect;

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, MATRIX_4X4_IDENTITY)
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

        let xs = intersect(&s, &r).unwrap();

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0)).unwrap();
        let mut s = Sphere::new();
        s.transform(scaling(5.0, 0.0, 0.0));

        let xs = intersect(&s, &r);

        assert!(xs.is_none());
    }
}