use crate::color::Color;
use crate::pattern::{Pattern, PatternProps};
use crate::tuple::Tuple;

#[derive(Debug)]
pub(crate) struct CheckerPattern {
    props: PatternProps,
    a: Color,
    b: Color,
}

impl CheckerPattern {
    pub(crate) fn new(a: Color, b: Color) -> CheckerPattern {
        CheckerPattern {
            props: PatternProps::default(),
            a,
            b,
        }
    }
}

impl Pattern for CheckerPattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }

    fn get_props(&self) -> &PatternProps {
        &self.props
    }

    fn mut_props(&mut self) -> &mut PatternProps {
        &mut self.props
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::tuple::Tuple;

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = CheckerPattern::new(Color::new(1., 1., 1.), Color::new(0., 0., 0.));

        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0., 0.))
        );
        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0.99, 0., 0.))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(1.01, 0., 0.))
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = CheckerPattern::new(Color::new(1., 1., 1.), Color::new(0., 0., 0.));

        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0., 0.))
        );
        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0.99, 0.))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(0., 1.01, 0.))
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = CheckerPattern::new(Color::new(1., 1., 1.), Color::new(0., 0., 0.));

        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0., 0.))
        );
        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0., 0.99))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(0., 0., 1.01))
        );
    }
}
