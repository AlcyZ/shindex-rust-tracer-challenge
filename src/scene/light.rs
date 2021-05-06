use crate::math::tuple::Tuple;
use crate::scene::color::Color;

#[derive(Copy, Clone, Debug)]
pub(crate) struct PointLight {
    pub(crate) position: Tuple,
    pub(crate) intensity: Color,
}

impl PointLight {
    pub(crate) fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::tuple::Tuple;
    use crate::scene::color::Color;

    #[test]
    fn test_point_light_has_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Tuple::point(0., 0., 0.);
        let light = PointLight::new(position, intensity);

        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
