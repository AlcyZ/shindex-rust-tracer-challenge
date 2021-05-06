use crate::math::tuple::Tuple;
use crate::pattern::{Pattern, PatternProps};
use crate::scene::shading::color::Color;

#[derive(Debug)]
pub(crate) struct GradientPattern {
    props: PatternProps,
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub(crate) fn new(a: Color, b: Color) -> GradientPattern {
        GradientPattern {
            props: PatternProps::default(),
            a,
            b,
        }
    }
}

impl Pattern for GradientPattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
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
    use crate::math::tuple::Tuple;
    use crate::scene::shading::color::Color;

    #[test]
    fn test_gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(Color::new(1., 1., 1.), Color::new(0., 0., 0.));

        assert_eq!(
            pattern.pattern_at(Tuple::point(0., 0., 0.)),
            Color::new(1., 1., 1.)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
