use crate::matrix::{Matrix4x4, MATRIX_4X4_IDENTITY};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[0][3] = x;
    t_matrix[1][3] = y;
    t_matrix[2][3] = z;

    t_matrix
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[0][0] = x;
    t_matrix[1][1] = y;
    t_matrix[2][2] = z;

    t_matrix
}

pub fn rotation_x(radiant: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[1][1] = radiant.cos();
    t_matrix[1][2] = -radiant.sin();
    t_matrix[2][1] = radiant.sin();
    t_matrix[2][2] = radiant.cos();

    t_matrix
}

pub fn rotation_y(radiant: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[0][0] = radiant.cos();
    t_matrix[0][2] = radiant.sin();
    t_matrix[2][0] = -radiant.sin();
    t_matrix[2][2] = radiant.cos();

    t_matrix
}

pub fn rotation_z(radiant: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[0][0] = radiant.cos();
    t_matrix[0][1] = -radiant.sin();
    t_matrix[1][0] = radiant.sin();
    t_matrix[1][1] = radiant.cos();

    t_matrix
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
    let mut t_matrix = MATRIX_4X4_IDENTITY;

    t_matrix[0][1] = xy;
    t_matrix[0][2] = xz;
    t_matrix[1][0] = yx;
    t_matrix[1][2] = yz;
    t_matrix[2][0] = zx;
    t_matrix[2][1] = zy;

    t_matrix
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::matrix::{inverse, mul_by_tuple};
    use crate::transformation::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation};
    use crate::tuple::{point, tuple_eq, tuple_is_point, vector};

    #[test]
    fn multiply_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        let expected = point(2.0, 1.0, 7.0);
        let actual = mul_by_tuple(transform, p);

        assert_eq!(actual, expected);
        assert!(tuple_is_point(actual))
    }

    #[test]
    fn multiply_by_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = inverse(transform).unwrap();
        let p = point(-3.0, 4.0, 5.0);

        let expected = point(-8.0, 7.0, 3.0);
        let actual = mul_by_tuple(inv, p);

        assert_eq!(actual, expected);
        assert!(tuple_is_point(actual))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        let actual = mul_by_tuple(transform, v);

        assert_eq!(actual, v)
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let expected = point(-8.0, 18.0, 32.0);
        let actual = mul_by_tuple(transform, p);

        assert_eq!(actual, expected)
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let expected = vector(-8.0, 18.0, 32.0);
        let actual = mul_by_tuple(transform, v);

        assert_eq!(actual, expected)
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = inverse(transform).unwrap();
        let v = vector(-4.0, 6.0, 8.0);

        let expected = vector(-2.0, 2.0, 2.0);
        let actual = mul_by_tuple(inv, v);

        assert_eq!(actual, expected)
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let expected = point(-2.0, 3.0, 4.0);
        let actual = mul_by_tuple(transform, p);

        assert_eq!(actual, expected)
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let actual_half = mul_by_tuple(half_quarter, p);
        let actual_full = mul_by_tuple(full_quarter, p);

        let a = 2_f64.sqrt() / 2_f64;
        let expected_half = point(0.0, a, a);
        let expected_full = point(0.0, 0.0, 1.0);

        assert_eq!(actual_half, expected_half);
        assert!(tuple_eq(actual_full, expected_full));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let actual_half = mul_by_tuple(half_quarter, p);
        let actual_full = mul_by_tuple(full_quarter, p);

        let a = 2_f64.sqrt() / 2_f64;
        let expected_half = point(a, 0.0, a);
        let expected_full = point(1.0, 0.0, 0.0);

        assert_eq!(actual_half, expected_half);
        assert!(tuple_eq(actual_full, expected_full));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let actual_half = mul_by_tuple(half_quarter, p);
        let actual_full = mul_by_tuple(full_quarter, p);

        let a = 2_f64.sqrt() / 2_f64;
        let expected_half = point(-a, a, 0.0);
        let expected_full = point(-1.0, 0.0, 0.0);

        assert_eq!(actual_half, expected_half);
        assert!(tuple_eq(actual_full, expected_full));
    }

    #[test]
    fn shearing_of_point() {
        let p = point(2.0, 3.0, 4.0);
        let t_xy = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let t_xz = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let t_yx = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let t_yz = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let t_zx = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let t_zy = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let e_xy = point(5.0, 3.0, 4.0);
        let e_xz = point(6.0, 3.0, 4.0);
        let e_yx = point(2.0, 5.0, 4.0);
        let e_yz = point(2.0, 7.0, 4.0);
        let e_zx = point(2.0, 3.0, 6.0);
        let e_zy = point(2.0, 3.0, 7.0);

        assert_eq!(mul_by_tuple(t_xy, p), e_xy);
        assert_eq!(mul_by_tuple(t_xz, p), e_xz);
        assert_eq!(mul_by_tuple(t_yx, p), e_yx);
        assert_eq!(mul_by_tuple(t_yz, p), e_yz);
        assert_eq!(mul_by_tuple(t_zx, p), e_zx);
        assert_eq!(mul_by_tuple(t_zy, p), e_zy)
    }
}