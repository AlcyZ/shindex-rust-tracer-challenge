use crate::util::f64_eq;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1_f64)
    }

    pub fn origin_point() -> Tuple {
        Tuple::point(0.0, 0.0, 0.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0_f64)
    }

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1_f64
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0_f64
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn reflect(&self, other: Tuple) -> Tuple {
        return *self - other * 2.0 * self.dot(other);
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();

        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(self.y * other.z - self.z * other.y,
                      self.z * other.x - self.x * other.z,
                      self.x * other.y - self.y * other.x)
    }
}

impl std::cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.x, other.x) &&
            f64_eq(self.y, other.y) &&
            f64_eq(self.z, other.z) &&
            self.w == other.w
    }
}

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}


impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a = Tuple::new(1_f64, 2_f64, 3_f64, 1_f64);

        assert_eq!(1_f64, a.x);
        assert_eq!(2_f64, a.y);
        assert_eq!(3_f64, a.z);
        assert_eq!(1_f64, a.w);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a = Tuple::new(1_f64, 2_f64, 3_f64, 0_f64);

        assert_eq!(1_f64, a.x);
        assert_eq!(2_f64, a.y);
        assert_eq!(3_f64, a.z);
        assert_eq!(0_f64, a.w);
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    #[test]
    fn point_fn_creates_point() {
        let a = Tuple::new(1_f64, 2_f64, 3_f64, 1_f64);
        let b = Tuple::point(1_f64, 2_f64, 3_f64);

        assert_eq!(b, a)
    }

    #[test]
    fn vector_fn_creates_vector() {
        let a = Tuple::new(1_f64, 2_f64, 3_f64, 0_f64);
        let b = Tuple::vector(1_f64, 2_f64, 3_f64);

        assert_eq!(b, a)
    }

    #[test]
    fn adding_two_tuples() {
        let a = Tuple::new(2_f64, 3_f64, -4_f64, 1_f64);
        let b = Tuple::new(1_f64, 2_f64, 3_f64, 0_f64);
        let c = Tuple::new(3_f64, 5_f64, -1_f64, 1_f64);

        assert_eq!(c, a + b)
    }

    #[test]
    fn subtracting_two_tuples() {
        let a = Tuple::new(2_f64, 3_f64, -4_f64, 1_f64);
        let b = Tuple::new(1_f64, 2_f64, 3_f64, 0_f64);
        let c = Tuple::new(1_f64, 1_f64, -7_f64, 1_f64);

        assert_eq!(c, a - b)
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1_f64, -2_f64, 3_f64, -4_f64);
        let b = Tuple::new(-1_f64, 2_f64, -3_f64, 4_f64);

        assert_eq!(b, -a)
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 3.5;
        let c = a * b;
        let d = Tuple::new(3.5, -7_f64, 10.5, -14_f64);

        assert_eq!(d, c)
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = Tuple::new(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 0.5;
        let c = Tuple::new(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, a * b)
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 2_f64;
        let c = Tuple::new(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, a / b)
    }

    #[test]
    fn computing_magnitude_of_vector() {
        let a = Tuple::vector(1_f64, 0_f64, 0_f64);
        let b = Tuple::vector(0_f64, 1_f64, 0_f64);
        let c = Tuple::vector(0_f64, 0_f64, 1_f64);
        let d = Tuple::vector(1_f64, 2_f64, 3_f64);
        let e = Tuple::vector(-1_f64, -2_f64, -3_f64);

        let f = 14_f64;

        assert_eq!(1_f64, a.magnitude());
        assert_eq!(1_f64, b.magnitude());
        assert_eq!(1_f64, c.magnitude());
        assert_eq!(d.magnitude(), f.sqrt());
        assert_eq!(e.magnitude(), f.sqrt());
    }

    #[test]
    fn normalize_vector_one() {
        let a = Tuple::vector(4_f64, 0_f64, 0_f64);
        let expected = Tuple::vector(1_f64, 0_f64, 0_f64);
        let actual = a.normalize();

        assert_eq!(expected, actual);
    }

    #[test]
    fn normalize_vector_two() {
        let a = Tuple::vector(1_f64, 2_f64, 3_f64);

        let expected = Tuple::vector(1_f64 / 14_f64.sqrt(), 2_f64 / 14_f64.sqrt(), 3_f64 / 14_f64.sqrt());
        let actual = a.normalize();

        assert_eq!(expected, actual);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let a = Tuple::vector(1_f64, 2_f64, 3_f64);
        let normalized = a.normalize();

        assert_eq!(1_f64, normalized.magnitude());
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1_f64, 2_f64, 3_f64);
        let b = Tuple::vector(2_f64, 3_f64, 4_f64);
        let c = a.dot(b);

        assert_eq!(20_f64, c)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1_f64, 2_f64, 3_f64);
        let b = Tuple::vector(2_f64, 3_f64, 4_f64);
        let c = Tuple::vector(-1_f64, 2_f64, -1_f64);
        let d = Tuple::vector(1_f64, -2_f64, 1_f64);
        let e = a.cross(b);
        let f = b.cross(a);

        assert_eq!(e, c);
        assert_eq!(d, f);
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degree() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let a = 2_f64.sqrt() / 2.0;
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(a, a, 0.0);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
