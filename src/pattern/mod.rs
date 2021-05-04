use crate::color::Color;
use crate::math::matrix::M4;
use crate::shape::Shape;
use crate::tuple::Tuple;
use std::fmt::Debug;

pub(crate) mod checker;
pub(crate) mod gradient;
pub(crate) mod ring;
pub(crate) mod stripe;

pub(crate) trait Pattern: Debug + Sync + Send {
    fn pattern_at_shape(&self, shape: &dyn Shape, point: Tuple) -> Color {
        let object_point = shape.get_props().get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_props().get_transform().inverse().unwrap() * object_point;

        self.pattern_at(pattern_point)
    }

    fn pattern_at(&self, point: Tuple) -> Color;

    fn get_props(&self) -> &PatternProps;

    fn mut_props(&mut self) -> &mut PatternProps;
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct PatternProps {
    transform: M4,
}

impl PatternProps {
    fn default() -> PatternProps {
        PatternProps {
            transform: M4::identity(),
        }
    }

    fn get_transform(&self) -> M4 {
        self.transform
    }

    pub(crate) fn set_transform(&mut self, new: M4) {
        self.transform = new
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::math::transformation::{scaling, translation};
    use crate::shape::Shape;
    use crate::sphere::Sphere;

    #[derive(Copy, Clone, Debug)]
    struct TestPattern {
        props: PatternProps,
    }

    impl Pattern for TestPattern {
        fn pattern_at(&self, point: Tuple) -> Color {
            Color::new(point.x, point.y, point.z)
        }

        fn get_props(&self) -> &PatternProps {
            &self.props
        }

        fn mut_props(&mut self) -> &mut PatternProps {
            &mut self.props
        }
    }

    fn test_pattern() -> TestPattern {
        TestPattern {
            props: PatternProps::default(),
        }
    }

    #[test]
    fn test_default_pattern_transformation() {
        let pattern = test_pattern();

        assert_eq!(pattern.get_props().get_transform(), M4::identity())
    }

    #[test]
    fn test_assign_pattern_transformation() {
        let mut pattern = test_pattern();
        pattern.mut_props().set_transform(translation(1., 2., 3.));

        assert_eq!(pattern.get_props().get_transform(), translation(1., 2., 3.))
    }

    #[test]
    fn test_pattern_with_object_transformation() {
        let mut shape = Sphere::new();
        shape.mut_props().set_transform(scaling(2., 2., 2.));

        let pattern = test_pattern();
        let c = pattern.pattern_at_shape(&shape, Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn test_pattern_with_pattern_transformation() {
        let shape = Sphere::new();
        let mut pattern = test_pattern();
        pattern.mut_props().set_transform(scaling(2., 2., 2.));

        let c = pattern.pattern_at_shape(&shape, Tuple::point(2., 3., 4.));

        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn test_pattern_with_object_and_pattern_transformation() {
        let mut shape = Sphere::new();
        shape.mut_props().set_transform(scaling(2., 2., 2.));

        let mut pattern = test_pattern();
        pattern.mut_props().set_transform(translation(0.5, 1., 1.5));

        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.5, 3., 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
