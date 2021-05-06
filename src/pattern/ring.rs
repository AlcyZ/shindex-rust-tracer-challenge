use crate::color::Color;
use crate::math::f64_eq;
use crate::pattern::{Pattern, PatternProps};
use crate::tuple::Tuple;

#[derive(Debug)]
pub(crate) struct RingPattern {
    props: PatternProps,
    a: Color,
    b: Color,
}

impl RingPattern {
    pub(crate) fn new(a: Color, b: Color) -> RingPattern {
        RingPattern {
            props: PatternProps::default(),
            a,
            b,
        }
    }
}

impl Pattern for RingPattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        if f64_eq(point.x.powi(2) + point.z.powi(2).sqrt().floor() % 2., 0.) {
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
    fn test_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(Color::new(1., 1., 1.), Color::new(0., 0., 0.));

        assert_eq!(
            Color::new(1., 1., 1.),
            pattern.pattern_at(Tuple::point(0., 0., 0.))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(1., 0., 0.))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(0., 0., 1.))
        );
        assert_eq!(
            Color::new(0., 0., 0.),
            pattern.pattern_at(Tuple::point(0.708, 0., 0.708))
        );
    }
}
