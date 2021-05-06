use crate::math::f64_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Tuple {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) w: f64,
}

impl Tuple {
    pub(crate) fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub(crate) fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.)
    }

    pub(crate) fn direction(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.)
    }

    pub(crate) fn is_point(&self) -> bool {
        return self.w == 1.;
    }

    pub(crate) fn is_direction(&self) -> bool {
        return self.w == 0.;
    }

    pub(crate) fn magnitude(&self) -> f64 {
        let total = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);

        total.sqrt()
    }

    pub(crate) fn normalize(&self) -> Tuple {
        let x = self.x / self.magnitude();
        let y = self.y / self.magnitude();
        let z = self.z / self.magnitude();
        let w = self.w / self.magnitude();

        Tuple::new(x, y, z, w)
    }

    pub(crate) fn dot(&self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub(crate) fn cross(&self, other: Tuple) -> Tuple {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Tuple::direction(x, y, z)
    }

    pub(crate) fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2. * self.dot(normal)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.x, other.x)
            && f64_eq(self.y, other.y)
            && f64_eq(self.z, other.z)
            && self.w == other.w
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;

        Tuple::new(x, y, z, w)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;

        Tuple::new(x, y, z, w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        let w = -self.w;

        Tuple::new(x, y, z, w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;

        Tuple::new(x, y, z, w)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        let w = self.w / rhs;

        Tuple::new(x, y, z, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_with_w_equals_1_is_point() {
        let p: Tuple = Tuple::new(4.3, -4.2, 3.1, 1.);

        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert!(p.is_point());
        assert!(!p.is_direction());
    }

    #[test]
    fn test_tuple_with_w_equals_0_is_direction() {
        let d = Tuple::new(4.3, -4.2, 3.1, 0.);

        assert_eq!(d.x, 4.3);
        assert_eq!(d.y, -4.2);
        assert_eq!(d.z, 3.1);
        assert!(d.is_direction());
        assert!(!d.is_point());
    }

    #[test]
    fn test_point_has_factory_fn() {
        let p: Tuple = Tuple::point(4.3, -4.2, 3.1);
        let c: Tuple = Tuple::new(4.3, -4.2, 3.1, 1.);

        assert_eq!(p, c);
    }

    #[test]
    fn test_direction_has_factory_fn() {
        let p: Tuple = Tuple::direction(4.3, -4.2, 3.1);
        let c: Tuple = Tuple::new(4.3, -4.2, 3.1, 0.);

        assert_eq!(p, c);
    }

    #[test]
    fn test_add_two_tuples() {
        let a = Tuple::new(3., -2., 5., 1.);
        let b = Tuple::new(-2., 3., 1., 0.);

        let e = Tuple::new(1., 1., 6., 1.);
        let r = a + b;

        assert_eq!(e, r);
    }

    #[test]
    fn test_subtract_two_points() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::point(5., 6., 7.);

        let e = Tuple::direction(-2., -4., -6.);
        let r = a - b;

        assert_eq!(e, r);
    }

    #[test]
    fn test_subtract_direction_from_point() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::direction(5., 6., 7.);

        let e = Tuple::point(-2., -4., -6.);
        let r = a - b;

        assert_eq!(e, r);
    }

    #[test]
    fn test_subtract_two_directions() {
        let a = Tuple::direction(3., 2., 1.);
        let b = Tuple::direction(5., 6., 7.);

        let e = Tuple::direction(-2., -4., -6.);
        let r = a - b;

        assert_eq!(e, r);
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1., 2., 3., -4.);

        let e = Tuple::new(-1., -2., -3., 4.);
        let r = -a;

        assert_eq!(e, r);
    }

    #[test]
    fn test_mul_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);

        let e = Tuple::new(3.5, -7., 10.5, -14.);
        let r = a * 3.5;

        assert_eq!(e, r);
    }

    #[test]
    fn test_mul_tuple_by_faction() {
        let a = Tuple::new(1., -2., 3., -4.);

        let e = Tuple::new(0.5, -1., 1.5, -2.);
        let r = a * 0.5;

        assert_eq!(e, r);
    }

    #[test]
    fn test_div_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);

        let e = Tuple::new(0.5, -1., 1.5, -2.);
        let r = a / 2.;

        assert_eq!(e, r);
    }

    #[test]
    fn test_compute_magnitude_1() {
        let d = Tuple::direction(1., 0., 0.);

        let e = 1.;
        let r = d.magnitude();

        assert_eq!(e, r);
    }

    #[test]
    fn test_compute_magnitude_2() {
        let d = Tuple::direction(0., 1., 0.);

        let e = 1.;
        let r = d.magnitude();

        assert_eq!(e, r);
    }

    #[test]
    fn test_compute_magnitude_3() {
        let d = Tuple::direction(0., 0., 1.);

        let e = 1.;
        let r = d.magnitude();

        assert_eq!(e, r);
    }

    #[test]
    fn test_compute_magnitude_4() {
        let d = Tuple::direction(1., 2., 3.);

        let e = (14_f64).sqrt();
        let r = d.magnitude();

        assert_eq!(e, r);
    }

    #[test]
    fn test_compute_magnitude_5() {
        let d = Tuple::direction(-1., -2., -3.);

        let e = (14_f64).sqrt();
        let r = d.magnitude();

        assert_eq!(e, r);
    }

    #[test]
    fn test_normalize_1() {
        let d = Tuple::direction(4., 0., 0.);

        let e = Tuple::direction(1., 0., 0.);
        let r = d.normalize();

        assert_eq!(e, r);
    }

    #[test]
    fn test_normalize_2() {
        let d = Tuple::direction(1., 2., 3.);

        let e = Tuple::direction(0.26726, 0.53452, 0.80178);
        let r = d.normalize();

        assert_eq!(e, r);
    }

    #[test]
    fn test_magnitude_of_normalized_vector() {
        let d = Tuple::direction(1., 2., 3.);
        let n = d.normalize();

        let r = n.magnitude();
        let e = 1f64;

        assert!(f64_eq(e, r));
    }

    #[test]
    fn test_dot_product_of_directions() {
        let a = Tuple::direction(1., 2., 3.);
        let b = Tuple::direction(2., 3., 4.);

        let e = 20f64;
        let r = a.dot(b);

        assert_eq!(e, r);
    }

    #[test]
    fn test_cross_product_1() {
        let a = Tuple::direction(1., 2., 3.);
        let b = Tuple::direction(2., 3., 4.);

        let e = Tuple::direction(-1., 2., -1.);
        let r = a.cross(b);

        assert_eq!(e, r);
    }

    #[test]
    fn test_cross_product_2() {
        let a = Tuple::direction(1., 2., 3.);
        let b = Tuple::direction(2., 3., 4.);

        let e = Tuple::direction(1., -2., 1.);
        let r = b.cross(a);

        assert_eq!(e, r);
    }

    #[test]
    fn test_reflecting_direction_approaching_at_45_degree() {
        let d = Tuple::direction(1., -1., 0.);
        let n = Tuple::direction(0., 1., 0.);

        let e = Tuple::direction(1., 1., 0.);
        assert_eq!(e, d.reflect(n))
    }

    #[test]
    fn test_reflecting_direction_off_slanted_surface() {
        let d = Tuple::direction(0., -1., 0.);
        let n = Tuple::direction(2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.);

        let e = Tuple::direction(1., 0., 0.);
        assert_eq!(e, d.reflect(n))
    }
}
