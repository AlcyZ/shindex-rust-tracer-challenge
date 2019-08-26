use crate::tuple::{Tuple, dot};

type Matrix4x4 = [[f64; 4]; 4];

const MATRIX_4X4_IDENTITY: Matrix4x4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

fn matrix_4x4_mul(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
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

fn matrix_4x4_mul_t(a: Matrix4x4, b: Tuple) -> Tuple {
    let mut t = [0.0, 0.0, 0.0, 0.0];

    for row in 0..4 {
        let a = a[row];
        t[row] = dot(&a, &b);
    }

    t
}

fn matrix_transpose(m: Matrix4x4) -> Matrix4x4 {
    let mut new: Matrix4x4 = [[0.0; 4]; 4];

    for row in 0..4 {
        for col in 0..4 {
            new[row][col] = m[col][row];
        }
    }

    new
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
        let actual = matrix_4x4_mul(a, b);
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
        let actual = matrix_4x4_mul_t(a, b);
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
        let actual = matrix_4x4_mul(a, MATRIX_4X4_IDENTITY);
        assert_eq!(actual, a)
    }


    #[test]
    fn multiply_tuple_by_identity() {
        let a: Tuple = [1.0, 2.0, 3.0, 4.0];
        let actual = matrix_4x4_mul_t(MATRIX_4X4_IDENTITY, a);
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
        let actual = matrix_transpose(a);
        assert_eq!(actual, expected)
    }
}


