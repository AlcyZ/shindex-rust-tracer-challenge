use crate::math::f64_eq;
use crate::math::tuple::Tuple;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub(crate) struct M4 {
    data: [f64; 16],
}

impl M4 {
    pub(crate) fn identity() -> M4 {
        M4 {
            data: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn get(&self, row: usize, column: usize) -> f64 {
        let multiplier = row * 4;

        self.data[multiplier + column]
    }

    pub(crate) fn set(&mut self, value: f64, row: usize, col: usize) {
        let index = row * 4 + col;
        if index > 15 {
            return;
        }

        self.data[index] = value;
    }

    pub(crate) fn transpose(&self) -> M4 {
        let mut data = [0.; 16];

        for r in 0..4 {
            for c in 0..4 {
                data[r * 4 + c] = self.get(c, r);
            }
        }

        M4::from(data)
    }

    fn submatrix(&self, row: usize, col: usize) -> M3 {
        let mut data = [0.; 9];

        let mut i = 0;
        for r in 0..4 {
            if r == row {
                continue;
            }

            let mut j = 0;
            for c in 0..4 {
                if c == col {
                    continue;
                }
                data[i * 3 + j] = self.get(r, c);

                j += 1;
            }
            i += 1;
        }

        M3::from(data)
    }

    fn determinant(&self) -> f64 {
        let mut d = 0.;

        for c in 0..4 {
            d = d + self.get(0, c) * self.cofactor(0, c);
        }

        d
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    pub(crate) fn inverse(&self) -> Option<M4> {
        if !self.is_invertible() {
            return None;
        }
        let mut data = [0.; 16];

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                data[col * 4 + row] = c / self.determinant();
            }
        }

        Some(M4::from(data))
    }
}

impl Matrix for M4 {
    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
}

impl Mul for M4 {
    type Output = M4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = M4::from([0.; 16]);

        for r in 0..4 {
            for c in 0..4 {
                result.data[r * 4 + c] = self.get(r, 0) * rhs.get(0, c)
                    + self.get(r, 1) * rhs.get(1, c)
                    + self.get(r, 2) * rhs.get(2, c)
                    + self.get(r, 3) * rhs.get(3, c);
            }
        }

        result
    }
}

impl Mul<Tuple> for M4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result = [0.; 4];

        for r in 0..4 {
            let t = Tuple::new(
                self.get(r, 0),
                self.get(r, 1),
                self.get(r, 2),
                self.get(r, 3),
            );

            result[r] = t.dot(rhs);
        }

        Tuple::new(result[0], result[1], result[2], result[3])
    }
}

impl From<[f64; 16]> for M4 {
    fn from(data: [f64; 16]) -> Self {
        M4 { data }
    }
}

impl PartialEq for M4 {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.data[0], other.data[0])
            && f64_eq(self.data[1], other.data[1])
            && f64_eq(self.data[2], other.data[2])
            && f64_eq(self.data[3], other.data[3])
            && f64_eq(self.data[4], other.data[4])
            && f64_eq(self.data[5], other.data[5])
            && f64_eq(self.data[6], other.data[6])
            && f64_eq(self.data[7], other.data[7])
            && f64_eq(self.data[8], other.data[8])
            && f64_eq(self.data[9], other.data[9])
            && f64_eq(self.data[10], other.data[10])
            && f64_eq(self.data[11], other.data[11])
            && f64_eq(self.data[12], other.data[12])
            && f64_eq(self.data[13], other.data[13])
            && f64_eq(self.data[14], other.data[14])
            && f64_eq(self.data[15], other.data[15])
    }
}

#[derive(Copy, Clone, Debug)]
struct M3 {
    data: [f64; 9],
}

impl M3 {
    fn get(&self, row: usize, column: usize) -> f64 {
        let multiplier = row * 3;

        self.data[multiplier + column]
    }

    fn submatrix(&self, row: usize, col: usize) -> M2 {
        let mut data = [0.; 4];

        let mut i = 0;
        for r in 0..3 {
            if r == row {
                continue;
            }

            let mut j = 0;
            for c in 0..3 {
                if c == col {
                    continue;
                }
                data[i * 2 + j] = self.get(r, c);

                j += 1;
            }
            i += 1;
        }

        M2::from(data)
    }

    fn determinant(&self) -> f64 {
        let mut d = 0.;

        for c in 0..3 {
            d = d + self.get(0, c) * self.cofactor(0, c);
        }

        d
    }
}

impl Matrix for M3 {
    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
}

impl From<[f64; 9]> for M3 {
    fn from(data: [f64; 9]) -> Self {
        M3 { data }
    }
}

impl PartialEq for M3 {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.data[0], other.data[0])
            && f64_eq(self.data[1], other.data[1])
            && f64_eq(self.data[2], other.data[2])
            && f64_eq(self.data[3], other.data[3])
            && f64_eq(self.data[4], other.data[4])
            && f64_eq(self.data[5], other.data[5])
            && f64_eq(self.data[6], other.data[6])
            && f64_eq(self.data[7], other.data[7])
            && f64_eq(self.data[8], other.data[8])
    }
}

#[derive(Copy, Clone, Debug)]
struct M2 {
    data: [f64; 4],
}

impl M2 {
    fn get(&self, row: usize, column: usize) -> f64 {
        let multiplier = row * 2;

        self.data[multiplier + column]
    }

    fn determinant(&self) -> f64 {
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }
}

impl From<[f64; 4]> for M2 {
    fn from(data: [f64; 4]) -> Self {
        M2 { data }
    }
}

impl PartialEq for M2 {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.data[0], other.data[0])
            && f64_eq(self.data[1], other.data[1])
            && f64_eq(self.data[2], other.data[2])
            && f64_eq(self.data[3], other.data[3])
    }
}

trait Matrix {
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);

        if (row + col) as i32 & 1 == 1 {
            -minor
        } else {
            minor
        }
    }

    fn minor(&self, row: usize, col: usize) -> f64;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::tuple::Tuple;

    #[test]
    fn test_create_and_access_m4_matrix() {
        let m = M4::from([
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(1., m.get(0, 0));
        assert_eq!(4., m.get(0, 3));
        assert_eq!(5.5, m.get(1, 0));
        assert_eq!(7.5, m.get(1, 2));
        assert_eq!(11., m.get(2, 2));
        assert_eq!(13.5, m.get(3, 0));
        assert_eq!(15.5, m.get(3, 2));
    }

    #[test]
    fn test_create_and_access_m2_matrix() {
        let m = M2::from([-3., 5., 1., -2.]);

        assert_eq!(-3., m.get(0, 0));
        assert_eq!(5., m.get(0, 1));
        assert_eq!(1., m.get(1, 0));
        assert_eq!(-2., m.get(1, 1));
    }

    #[test]
    fn test_create_and_access_m3_matrix() {
        let m = M3::from([-3., 5., 0., 1., -2., -7., 0., 1., 1.]);

        assert_eq!(-3., m.get(0, 0));
        assert_eq!(-2., m.get(1, 1));
        assert_eq!(1., m.get(2, 2));
    }

    #[test]
    fn test_equality_with_identical_matrices() {
        let m1 = M4::from([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let m2 = M4::from([
            0.9999999999999999999,
            2.,
            3.,
            4.,
            5.,
            6.,
            7.,
            8.,
            9.,
            8.,
            7.,
            6.,
            5.,
            4.,
            3.,
            2.,
        ]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_equality_with_different_matrices() {
        let m1 = M4::from([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let m2 = M4::from([
            2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        ]);

        assert_ne!(m1, m2);
    }

    #[test]
    fn test_multiply_two_matrices() {
        let m1 = M4::from([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let m2 = M4::from([
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        ]);

        let e = M4::from([
            20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
        ]);
        let r = m1 * m2;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_matrix_by_tuple() {
        let m = M4::from([
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ]);
        let p = Tuple::new(1., 2., 3., 1.);

        let e = Tuple::new(18., 24., 33., 1.);
        let r = m * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_matrix_by_identity_matrix() {
        let m = M4::from([
            0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
        ]);
        let identity = M4::identity();
        let r = m * identity;

        assert_eq!(m, r);
    }

    #[test]
    fn test_multiply_matrix_by_identity_tuple() {
        let t = Tuple::new(1., 2., 3., 4.);
        let identity = M4::identity();
        let r = identity * t;

        assert_eq!(t, r);
    }

    #[test]
    fn test_transposing_matrix() {
        let m = M4::from([
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        ]);

        let e = M4::from([
            0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
        ]);
        let r = m.transpose();
        assert_eq!(e, r);
    }

    #[test]
    fn calc_determinant_of_2x2_matrix() {
        let m = M2::from([1., 5., -3., 2.]);

        assert_eq!(17., m.determinant())
    }

    #[test]
    fn test_submatrix_of_3x3() {
        let m = M3::from([1., 5., 0., -3., 2., 7., 0., 6., -3.]);

        let e = M2::from([-3., 2., 0., 6.]);
        let r = m.submatrix(0, 2);

        assert_eq!(e, r);
    }

    #[test]
    fn test_submatrix_of_4x4() {
        let m = M4::from([
            -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
        ]);

        let e = M3::from([-6., 1., 6., -8., 8., 6., -7., -1., 1.]);
        let r = m.submatrix(2, 1);

        assert_eq!(e, r);
    }

    #[test]
    fn test_calc_minor_of_3x3_matrix() {
        let m = M3::from([3., 5., 0., 2., -1., -7., 6., -1., 5.]);
        let b = m.submatrix(1, 0);

        assert_eq!(25., b.determinant());
        assert_eq!(25., m.minor(1, 0));
    }

    #[test]
    fn calc_cofactor_of_3x3_matrix() {
        let m = M3::from([3., 5., 0., 2., -1., -7., 6., -1., 5.]);

        assert_eq!(-12., m.minor(0, 0));
        assert_eq!(-12., m.cofactor(0, 0));
        assert_eq!(25., m.minor(1, 0));
        assert_eq!(-25., m.cofactor(1, 0));
    }

    #[test]
    fn calc_determinant_of_3x3_matrix() {
        let m = M3::from([1., 2., 6., -5., 8., -4., 2., 6., 4.]);

        assert_eq!(56., m.cofactor(0, 0));
        assert_eq!(12., m.cofactor(0, 1));
        assert_eq!(-46., m.cofactor(0, 2));
        assert_eq!(-196., m.determinant());
    }

    #[test]
    fn calc_determinant_of_4x4_matrix() {
        let m = M4::from([
            -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
        ]);

        assert_eq!(690., m.cofactor(0, 0));
        assert_eq!(447., m.cofactor(0, 1));
        assert_eq!(210., m.cofactor(0, 2));
        assert_eq!(51., m.cofactor(0, 3));
        assert_eq!(-4071., m.determinant());
    }

    #[test]
    fn test_invertible_matrix() {
        let m = M4::from([
            6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
        ]);

        assert_eq!(-2120., m.determinant());
        assert!(m.is_invertible());
    }

    #[test]
    fn test_non_invertible_matrix() {
        let m = M4::from([
            -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
        ]);

        assert_eq!(0., m.determinant());
        assert!(!m.is_invertible());
    }

    #[test]
    fn test_calc_inverse_of_matrix_1() {
        let m = M4::from([
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        ]);
        let e = M4::from([
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        ]);
        let r = m.inverse().unwrap();

        assert_eq!(532., m.determinant());
        assert_eq!(-160., m.cofactor(2, 3));
        assert_eq!(-160. / 532., r.get(3, 2));
        assert_eq!(105., m.cofactor(3, 2));
        assert_eq!(105. / 532., r.get(2, 3));
        assert_eq!(e, r);
    }

    #[test]
    fn test_calc_inverse_of_matrix_2() {
        let m = M4::from([
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        ]);

        let e = M4::from([
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ]);
        let r = m.inverse().unwrap();

        assert_eq!(e, r);
    }

    #[test]
    fn test_calc_inverse_of_matrix_3() {
        let m = M4::from([
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        ]);

        let e = M4::from([
            -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
            -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
        ]);
        let r = m.inverse().unwrap();

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_product_by_inverse() {
        let a = M4::from([
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        ]);
        let b = M4::from([
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        ]);
        let c = a * b;

        assert_eq!(a, c * b.inverse().unwrap())
    }
}
