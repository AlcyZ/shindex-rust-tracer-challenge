use crate::math::tuple::Tuple;
use crate::pattern::{Pattern, PatternProps};
use crate::scene::shading::color::Color;

#[derive(Debug)]
pub(crate) struct StripePattern {
    a: Color,
    b: Color,
    props: PatternProps,
}

impl Pattern for StripePattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2. == 0. {
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

impl StripePattern {
    pub(crate) fn new(a: Color, b: Color) -> StripePattern {
        StripePattern {
            a,
            b,
            props: PatternProps::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b_w_colors() -> (Color, Color) {
        (Color::new(0., 0., 0.), Color::new(1., 1., 1.))
    }

    #[test]
    fn test_create_stripe_pattern() {
        let (black, white) = b_w_colors();
        let p = StripePattern::new(white, black);

        assert_eq!(p.a, white);
        assert_eq!(p.b, black);
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_y() {
        let (black, white) = b_w_colors();
        let p = StripePattern::new(white, black);

        assert_eq!(white, p.pattern_at(Tuple::point(0., 0., 0.)));
        assert_eq!(white, p.pattern_at(Tuple::point(0., 1., 0.)));
        assert_eq!(white, p.pattern_at(Tuple::point(0., 2., 0.)));
    }

    #[test]
    fn test_stripe_pattern_is_constant_in_z() {
        let (black, white) = b_w_colors();
        let p = StripePattern::new(white, black);

        assert_eq!(white, p.pattern_at(Tuple::point(0., 0., 0.)));
        assert_eq!(white, p.pattern_at(Tuple::point(0., 0., 1.)));
        assert_eq!(white, p.pattern_at(Tuple::point(0., 0., 2.)));
    }

    #[test]
    fn test_stripe_pattern_alternates_in_x() {
        let (black, white) = b_w_colors();
        let p = StripePattern::new(white, black);

        assert_eq!(white, p.pattern_at(Tuple::point(0., 0., 0.)));
        assert_eq!(white, p.pattern_at(Tuple::point(0.9, 0., 0.)));
        assert_eq!(black, p.pattern_at(Tuple::point(1., 0., 0.)));
        assert_eq!(black, p.pattern_at(Tuple::point(-0.1, 0., 0.)));
        assert_eq!(black, p.pattern_at(Tuple::point(-1., 0., 0.)));
        assert_eq!(white, p.pattern_at(Tuple::point(-1.1, 0., 0.)));
    }
}
