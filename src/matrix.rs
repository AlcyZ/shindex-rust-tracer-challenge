use crate::tuple::{dot, Tuple};
use crate::util::f64_eq;

type Matrix4x4 = [[f64; 4]; 4];
type Matrix3x3 = [[f64; 3]; 3];
type Matrix2x2 = [[f64; 2]; 2];

const MATRIX_4X4_IDENTITY: Matrix4x4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

fn matrix_4x4_eq(a: Matrix4x4, b: Matrix4x4) -> bool {
    for (i, a_outer) in a.iter().enumerate() {
        for (j, a_inner) in a_outer.iter().enumerate() {
            if !f64_eq(*a_inner, b[i][j]) {
                return false;
            }
        }
    }

    true
}

fn mul(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
    let mut m: Matrix4x4 = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            m[row][col] = a[row][0] * b[0][col] +
                a[row][1] * b[1][col] +
                a[row][2] * b[2][col] +
                a[row][3] * b[3][col];
        }
    }
    m
}

fn mul_by_tuple(a: Matrix4x4, b: Tuple) -> Tuple {
    let mut t = [0.0, 0.0, 0.0, 0.0];

    for row in 0..4 {
        let a = a[row];
        t[row] = dot(a, b);
    }

    t
}

fn transpose(m: Matrix4x4) -> Matrix4x4 {
    let mut new: Matrix4x4 = [[0.0; 4]; 4];

    for row in 0..4 {
        for col in 0..4 {
            new[row][col] = m[col][row];
        }
    }

    new
}

fn determinant_2x2(m: Matrix2x2) -> f64 {
    m[0][0] * m[1][1] - m[1][0] * m[0][1]
}

fn determinant_3x3(m: Matrix3x3) -> f64 {
    let mut d = 0.0;

    for col in 0..3 {
        d = d + m[0][col] * cofactor_3x3(m, 0, col);
    }

    d
}

fn determinant_4x4(m: Matrix4x4) -> f64 {
    let mut d = 0.0;

    for col in 0..4 {
        d = d + m[0][col] * cofactor_4x4(m, 0, col);
    }

    d
}

fn minor_3x3(m: Matrix3x3, row: usize, col: usize) -> f64 {
    determinant_2x2(submatrix_3x3(m, row, col))
}

fn minor_4x4(m: Matrix4x4, row: usize, col: usize) -> f64 {
    determinant_3x3(submatrix_4x4(m, row, col))
}

fn cofactor_3x3(m: Matrix3x3, row: usize, col: usize) -> f64 {
    let minor = minor_3x3(m, row, col);
    let cf_identifier = row as i32 + col as i32;

    if cf_identifier & 1 == 1 {
        // is odd
        return -minor;
    }

    minor
}

fn cofactor_4x4(m: Matrix4x4, row: usize, col: usize) -> f64 {
    let minor = minor_4x4(m, row, col);
    let cf_identifier = row as i32 + col as i32;

    if cf_identifier & 1 == 1 {
        // is odd
        return -minor;
    }

    minor
}

fn submatrix_4x4(m: Matrix4x4, row: usize, col: usize) -> Matrix3x3 {
    let mut new: Matrix3x3 = [[0.0; 3]; 3];

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
            new[i][j] = m[r][c];
            j += 1;
        }
        i += 1;
    }

    new
}

fn submatrix_3x3(m: Matrix3x3, row: usize, col: usize) -> Matrix2x2 {
    let mut new: Matrix2x2 = [[0.0; 2]; 2];

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
            new[i][j] = m[r][c];
            j += 1;
        }
        i += 1;
    }

    new
}

fn is_invertible(m: Matrix4x4) -> bool {
    determinant_4x4(m) != 0_f64
}

fn inverse(m: Matrix4x4) -> Option<Matrix4x4> {
    let mut new: Matrix4x4;
    let determinant = determinant_4x4(m);

    if determinant == 0_f64 {
        return None;
    }
    new = [[0.0; 4]; 4];

    for row in 0..4 {
        for col in 0..4 {
            let cofactor = cofactor_4x4(m, row, col);

            new[col][row] = cofactor / determinant
        }
    }

    Some(new)
}

#[cfg(test)]
mod tests {
    use crate::matrix::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiply_matrix() {
        let a: Matrix4x4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b: Matrix4x4 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        let expected: Matrix4x4 = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];
        let actual = mul(a, b);
        assert_eq!(actual, expected)
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let a: Matrix4x4 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let b: Tuple = [1.0, 2.0, 3.0, 1.0];
        let expected = [18.0, 24.0, 33.0, 1.0];
        let actual = mul_by_tuple(a, b);
        assert_eq!(actual, expected)
    }


    #[test]
    fn multiply_matrix_by_identity() {
        let a: Matrix4x4 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let actual = mul(a, MATRIX_4X4_IDENTITY);
        assert_eq!(actual, a)
    }


    #[test]
    fn multiply_tuple_by_identity() {
        let a: Tuple = [1.0, 2.0, 3.0, 4.0];
        let actual = mul_by_tuple(MATRIX_4X4_IDENTITY, a);
        assert_eq!(actual, a)
    }

    #[test]
    fn transposing_matrix() {
        let a: Matrix4x4 = [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ];
        let expected: Matrix4x4 = [
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ];
        let actual = transpose(a);
        assert_eq!(actual, expected)
    }

    #[test]
    fn calc_determinant_of_2x2_matrix() {
        let a: Matrix2x2 = [
            [1.0, 5.0],
            [-3.0, 2.0],
        ];
        assert_eq!(determinant_2x2(a), 17.0)
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = [
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0],
        ];
        let expected_a = [
            [-3.0, 2.0],
            [0.0, 6.0],
        ];
        let b = [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];
        let expected_b = [
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0],
        ];
        let actual_a = submatrix_3x3(a, 0, 2);
        let actual_b = submatrix_4x4(b, 2, 1);

        assert_eq!(actual_a, expected_a);
        assert_eq!(actual_b, expected_b)
    }

    #[test]
    fn calc_minor_of_3x3_matrix() {
        let a: Matrix3x3 = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ];
        let b = submatrix_3x3(a, 1, 0);
        let expected = 25 as f64;

        let actual_determinant = determinant_2x2(b);
        let actual_minor = minor_3x3(a, 1, 0);

        assert_eq!(actual_determinant, expected);
        assert_eq!(actual_minor, expected)
    }

    #[test]
    fn calc_cofactor_of_matrix() {
        let a: Matrix3x3 = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ];

        assert_eq!(minor_3x3(a, 0, 0), -12 as f64);
        assert_eq!(cofactor_3x3(a, 0, 0), -12 as f64);
        assert_eq!(minor_3x3(a, 1, 0), 25 as f64);
        assert_eq!(cofactor_3x3(a, 1, 0), -25 as f64);
    }

    #[test]
    fn calc_determinant_of_3x3_matrix() {
        let a: Matrix3x3 = [
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0],
        ];

        assert_eq!(cofactor_3x3(a, 0, 0), 56_f64);
        assert_eq!(cofactor_3x3(a, 0, 1), 12_f64);
        assert_eq!(cofactor_3x3(a, 0, 2), -46_f64);
        assert_eq!(determinant_3x3(a), -196_f64);
    }

    #[test]
    fn calc_determinant_of_4x4_matrix() {
        let a: Matrix4x4 = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ];

        assert_eq!(cofactor_4x4(a, 0, 0), 690_f64);
        assert_eq!(cofactor_4x4(a, 0, 1), 447_f64);
        assert_eq!(cofactor_4x4(a, 0, 2), 210_f64);
        assert_eq!(cofactor_4x4(a, 0, 3), 51_f64);
        assert_eq!(determinant_4x4(a), -4071_f64);
    }

    #[test]
    fn invertible_matrix_test() {
        let a = [
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ];
        assert_eq!(determinant_4x4(a), -2120_f64);
        assert!(is_invertible(a));
    }

    #[test]
    fn non_invertible_matrix_test() {
        let a = [
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        assert_eq!(determinant_4x4(a), 0_f64);
        assert!(!is_invertible(a));
    }

    #[test]
    fn calc_inverse_of_matrix() {
        let a: Matrix4x4 = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ];
        let b = inverse(a).unwrap();
        let expected: Matrix4x4 = [
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ];

        assert_eq!(determinant_4x4(a), 532_f64);
        assert_eq!(cofactor_4x4(a, 2, 3), -160_f64);
        assert_eq!(b[3][2], -160_f64 / 532_f64);
        assert_eq!(cofactor_4x4(a, 3, 2), 105_f64);
        assert_eq!(b[2][3], 105_f64 / 532_f64);
    }

    #[test]
    fn calc_inverse_of_matrix_2() {
        let a = [
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ];

        let expected = [
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ];
        let actual = inverse(a).unwrap();

        assert!(matrix_4x4_eq(actual, expected))
    }
}
