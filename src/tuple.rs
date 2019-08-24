#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1_f64
    }
    fn is_vector(&self) -> bool {
        self.w == 0_f64
    }
}

impl std::cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;

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
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Mul for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

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
    type Output = Tuple;

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
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w,
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 1_f64,
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0_f64,
    }
}

pub fn normalize(v: &mut Tuple) {
    let mag = magnitude(v);

    v.x = v.x / mag;
    v.y = v.y / mag;
    v.z = v.z / mag;
    v.w = v.w / mag;
}

fn magnitude(v: &Tuple) -> f64 {
    (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

fn cross_vec(a: &Tuple, b: &Tuple) -> Tuple {
    vector(a.y * b.z - a.z * b.y,
           a.z * b.x - a.x * b.z,
           a.x * b.y - a.y * b.x)
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a = tuple(1_f64, 2_f64, 3_f64, 1_f64);

        assert_eq!(1_f64, a.x);
        assert_eq!(2_f64, a.y);
        assert_eq!(3_f64, a.z);
        assert_eq!(1_f64, a.w);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a = tuple(1_f64, 2_f64, 3_f64, 0_f64);

        assert_eq!(1_f64, a.x);
        assert_eq!(2_f64, a.y);
        assert_eq!(3_f64, a.z);
        assert_eq!(0_f64, a.w);
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    #[test]
    fn point_fn_creates_point() {
        let a = tuple(1_f64, 2_f64, 3_f64, 1_f64);
        let b = point(1_f64, 2_f64, 3_f64);

        assert_eq!(b, a)
    }

    #[test]
    fn vector_fn_creates_vector() {
        let a = tuple(1_f64, 2_f64, 3_f64, 0_f64);
        let b = vector(1_f64, 2_f64, 3_f64);

        assert_eq!(b, a)
    }

    #[test]
    fn adding_two_tuples() {
        let a = tuple(2_f64, 3_f64, -4_f64, 1_f64);
        let b = tuple(1_f64, 2_f64, 3_f64, 0_f64);
        let c = tuple(3_f64, 5_f64, -1_f64, 1_f64);

        assert_eq!(c, a + b)
    }

    #[test]
    fn subtracting_two_tuples() {
        let a = tuple(2_f64, 3_f64, -4_f64, 1_f64);
        let b = tuple(1_f64, 2_f64, 3_f64, 0_f64);
        let c = tuple(1_f64, 1_f64, -7_f64, 1_f64);

        assert_eq!(c, a - b)
    }

    #[test]
    fn negating_tuple() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = tuple(-1_f64, 2_f64, -3_f64, 4_f64);

        assert_eq!(b, -a)
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 3.5;
        let c = a * b;
        let d = tuple(3.5, -7_f64, 10.5, -14_f64);

        assert_eq!(d, c)
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 0.5;
        let c = tuple(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, a * b)
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 2_f64;
        let c = tuple(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, a / b)
    }

    #[test]
    fn computing_magnitude_of_vector() {
        let a = vector(1_f64, 0_f64, 0_f64);
        let b = vector(0_f64, 1_f64, 0_f64);
        let c = vector(0_f64, 0_f64, 1_f64);
        let d = vector(1_f64, 2_f64, 3_f64);
        let e = vector(-1_f64, -2_f64, -3_f64);

        let f = 14_f64;

        assert_eq!(1_f64, magnitude(&a));
        assert_eq!(1_f64, magnitude(&b));
        assert_eq!(1_f64, magnitude(&c));
        assert_eq!(magnitude(&d), f.sqrt());
        assert_eq!(magnitude(&e), f.sqrt());
    }

    #[test]
    fn normalize_vector_one() {
        let mut a = vector(4_f64, 0_f64, 0_f64);
        let b = vector(1_f64, 0_f64, 0_f64);
        normalize(&mut a);

        assert_eq!(b, a);
    }

    #[test]
    fn normalize_vector_two() {
        let mut a = vector(1_f64, 2_f64, 3_f64);

        let b = vector(1_f64 / 14_f64.sqrt(), 2_f64 / 14_f64.sqrt(), 3_f64 / 14_f64.sqrt());
        normalize(&mut a);

        assert_eq!(b, a);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let mut a = vector(1_f64, 2_f64, 3_f64);
        normalize(&mut a);

        assert_eq!(1_f64, magnitude(&a));
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = vector(1_f64, 2_f64, 3_f64);
        let b = vector(2_f64, 3_f64, 4_f64);
        let c = dot(&a, &b);

        assert_eq!(20_f64, c)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = vector(1_f64, 2_f64, 3_f64);
        let b = vector(2_f64, 3_f64, 4_f64);
        let c = vector(-1_f64, 2_f64, -1_f64);
        let d = vector(1_f64, -2_f64, 1_f64);
        let e = cross_vec(&a, &b);
        let f = cross_vec(&b, &a);

        assert_eq!(e, c);
        assert_eq!(d, f);
    }
}
