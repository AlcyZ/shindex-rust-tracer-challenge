pub type Tuple = [f64; 4];

pub fn tuple_is_point(t: Tuple) -> bool {
    t[3] == 1_f64
}

pub fn tuple_is_vector(t: Tuple) -> bool {
    t[3] == 0_f64
}

pub fn tuple_equals(a: Tuple, b: Tuple) -> bool {
    a[0] == b[0] && a[1] == b[1] && a[2] == b[2] && a[3] == b[3]
}

pub fn tuple_add(a: Tuple, b: Tuple) -> Tuple {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3],
    ]
}

pub fn tuple_subtract(a: Tuple, b: Tuple) -> Tuple {
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
        a[3] - b[3],
    ]
}

pub fn tuple_mul_scalar(a: Tuple, b: f64) -> Tuple {
    [
        a[0] * b,
        a[1] * b,
        a[2] * b,
        a[3] * b,
    ]
}

pub fn tuple_div_scalar(a: Tuple, b: f64) -> Tuple {
    [
        a[0] / b,
        a[1] / b,
        a[2] / b,
        a[3] / b,
    ]
}

pub fn tuple_neg(a: Tuple) -> Tuple {
    [
        -a[0],
        -a[1],
        -a[2],
        -a[3],
    ]
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    [x, y, z, w]
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    [x, y, z, 1_f64]
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    [x, y, z, 0_f64]
}

pub fn normalize(v: &mut Tuple) {
    let mag = magnitude(v);

    v[0] = v[0] / mag;
    v[1] = v[1] / mag;
    v[2] = v[2] / mag;
    v[3] = v[3] / mag;
}

fn magnitude(v: &Tuple) -> f64 {
    (v[0].powi(2) + v[1].powi(2) + v[2].powi(2) + v[3].powi(2)).sqrt()
}

fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

fn cross_vec(a: &Tuple, b: &Tuple) -> Tuple {
    vector(a[1] * b[2] - a[2] * b[1],
           a[2] * b[0] - a[0] * b[2],
           a[0] * b[1] - a[1] * b[0])
}

#[cfg(test)]
mod tests {
    use crate::tuple::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a = tuple(1_f64, 2_f64, 3_f64, 1_f64);

        assert_eq!(1_f64, a[0]);
        assert_eq!(2_f64, a[1]);
        assert_eq!(3_f64, a[2]);
        assert_eq!(1_f64, a[3]);
        assert!(tuple_is_point(a));
        assert!(!tuple_is_vector(a));
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a = tuple(1_f64, 2_f64, 3_f64, 0_f64);

        assert_eq!(1_f64, a[0]);
        assert_eq!(2_f64, a[1]);
        assert_eq!(3_f64, a[2]);
        assert_eq!(0_f64, a[3]);
        assert!(tuple_is_vector(a));
        assert!(!tuple_is_point(a));
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

        assert_eq!(c, tuple_add(a, b))
    }

    #[test]
    fn subtracting_two_tuples() {
        let a = tuple(2_f64, 3_f64, -4_f64, 1_f64);
        let b = tuple(1_f64, 2_f64, 3_f64, 0_f64);
        let c = tuple(1_f64, 1_f64, -7_f64, 1_f64);

        assert_eq!(c, tuple_subtract(a, b))
    }

    #[test]
    fn negating_tuple() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = tuple(-1_f64, 2_f64, -3_f64, 4_f64);

        assert_eq!(b, tuple_neg(a))
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 3.5;
        let c = tuple_mul_scalar(a, b);
        let d = tuple(3.5, -7_f64, 10.5, -14_f64);

        assert_eq!(d, c)
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 0.5;
        let c = tuple(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, tuple_mul_scalar(a, b))
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = tuple(1_f64, -2_f64, 3_f64, -4_f64);
        let b = 2_f64;
        let c = tuple(0.5, -1_f64, 1.5, -2_f64);

        assert_eq!(c, tuple_div_scalar(a, b))
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
