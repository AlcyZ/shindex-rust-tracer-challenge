use crate::math::matrix::M4;
use crate::math::tuple::Tuple;

pub(crate) fn translation(x: f64, y: f64, z: f64) -> M4 {
    let mut base = M4::identity();
    base.set(x, 0, 3);
    base.set(y, 1, 3);
    base.set(z, 2, 3);

    base
}

pub(crate) fn scaling(x: f64, y: f64, z: f64) -> M4 {
    let mut base = M4::identity();
    base.set(x, 0, 0);
    base.set(y, 1, 1);
    base.set(z, 2, 2);

    base
}

pub(crate) fn rotation_x(radians: f64) -> M4 {
    let mut base = M4::identity();
    base.set(radians.cos(), 1, 1);
    base.set(-radians.sin(), 1, 2);
    base.set(radians.sin(), 2, 1);
    base.set(radians.cos(), 2, 2);

    base
}

pub(crate) fn rotation_y(radians: f64) -> M4 {
    let mut base = M4::identity();
    base.set(radians.cos(), 0, 0);
    base.set(radians.sin(), 0, 2);
    base.set(-radians.sin(), 2, 0);
    base.set(radians.cos(), 2, 2);

    base
}

pub(crate) fn rotation_z(radians: f64) -> M4 {
    let mut base = M4::identity();
    base.set(radians.cos(), 0, 0);
    base.set(-radians.sin(), 0, 1);
    base.set(radians.sin(), 1, 0);
    base.set(radians.cos(), 1, 1);

    base
}

pub(crate) fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> M4 {
    let mut base = M4::identity();
    base.set(xy, 0, 1);
    base.set(xz, 0, 2);
    base.set(yx, 1, 0);
    base.set(yz, 1, 2);
    base.set(zx, 2, 0);
    base.set(zy, 2, 1);

    base
}

pub(crate) fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> M4 {
    let mut base = M4::identity();

    let forward_v = (to - from).normalize();
    let left_v = forward_v.cross(up.normalize());
    let true_up = left_v.cross(forward_v);

    base.set(left_v.x, 0, 0);
    base.set(left_v.y, 0, 1);
    base.set(left_v.z, 0, 2);
    base.set(true_up.x, 1, 0);
    base.set(true_up.y, 1, 1);
    base.set(true_up.z, 1, 2);
    base.set(-forward_v.x, 2, 0);
    base.set(-forward_v.y, 2, 1);
    base.set(-forward_v.z, 2, 2);

    base * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn test_multiply_by_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        let e = Tuple::point(2., 1., 7.);
        let r = transform * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_by_inverse_of_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let inverse = transform.inverse().unwrap();
        let p = Tuple::point(-3., 4., 5.);

        let e = Tuple::point(-8., 7., 3.);
        let r = inverse * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_translation_does_not_affect_direction_vectors() {
        let transform = translation(5., -3., 2.);
        let t = Tuple::direction(-3., 4., 5.);

        let r = transform * t;

        assert_eq!(t, r);
    }

    #[test]
    fn test_scaling_matrix_applied_to_point() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        let e = Tuple::point(-8., 18., 32.);
        let r = transform * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_scaling_matrix_applied_to_vector() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::direction(-4., 6., 8.);

        let e = Tuple::direction(-8., 18., 32.);
        let r = transform * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_inverse_of_scaling_matrix() {
        let transform = scaling(2., 3., 4.);
        let inverse = transform.inverse().unwrap();
        let p = Tuple::direction(-4., 6., 8.);

        let e = Tuple::direction(-2., 2., 2.);
        let r = inverse * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1., 1., 1.);
        let p = Tuple::direction(2., 3., 4.);

        let e = Tuple::direction(-2., 3., 4.);
        let r = transform * p;

        assert_eq!(e, r);
    }

    #[test]
    fn test_rotate_point_around_x_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let full_quarter = rotation_x(PI / 2.);

        assert_eq!(
            Tuple::point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(Tuple::point(0., 0., 1.), full_quarter * p);
    }

    #[test]
    fn test_inverse_rotate_point_around_x_axis_in_opposite_direction() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = rotation_x(PI / 4.);
        let inverse = half_quarter.inverse().unwrap();

        assert_eq!(
            Tuple::point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.),
            inverse * p
        );
    }

    #[test]
    fn test_rotate_point_around_y_axis() {
        let p = Tuple::point(0., 0., 1.);
        let half_quarter = rotation_y(PI / 4.);
        let full_quarter = rotation_y(PI / 2.);

        assert_eq!(
            Tuple::point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.),
            half_quarter * p
        );
        assert_eq!(Tuple::point(1., 0., 0.), full_quarter * p);
    }

    #[test]
    fn test_rotate_point_around_z_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = rotation_z(PI / 4.);
        let full_quarter = rotation_z(PI / 2.);

        assert_eq!(
            Tuple::point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.),
            half_quarter * p
        );
        assert_eq!(Tuple::point(-1., 0., 0.), full_quarter * p);
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1., 0., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(5., 3., 4.), transform * p);
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0., 1., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(6., 3., 4.), transform * p);
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0., 0., 1., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(2., 5., 4.), transform * p);
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0., 0., 0., 1., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(2., 7., 4.), transform * p);
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0., 0., 0., 0., 1., 0.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(2., 3., 6.), transform * p);
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0., 0., 0., 0., 0., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(Tuple::point(2., 3., 7.), transform * p);
    }

    #[test]
    fn test_individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);

        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1., -1., 0.));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5., -5., 0.));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15., 0., 7.));
    }

    #[test]
    fn test_chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1., 0., 1.);
        let a = rotation_x(PI / 2.);
        let b = scaling(5., 5., 5.);
        let c = translation(10., 5., 7.);

        let transformation = c * b * a;

        assert_eq!(transformation * p, Tuple::point(15., 0., 7.));
    }

    #[test]
    fn test_transformation_matrix_for_default_orientation() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::direction(0., 1., 0.);

        let t = view_transform(from, to, up);
        assert_eq!(t, M4::identity());
    }

    #[test]
    fn test_view_transformation_looking_in_positive_z_direction() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::direction(0., 1., 0.);

        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1., 1., -1.));
    }

    #[test]
    fn test_view_transformation_moves_world() {
        let from = Tuple::point(0., 0., 8.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::direction(0., 1., 0.);

        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0., 0., -8.));
    }

    #[test]
    fn test_view_in_arbitrary_direction() {
        let from = Tuple::point(1., 3., 2.);
        let to = Tuple::point(4., -2., 8.);
        let up = Tuple::direction(1., 1., 0.);

        let e = M4::from([
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843, -0.35857,
            0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
        ]);
        let r = view_transform(from, to, up);
        assert_eq!(e, r);
    }
}
