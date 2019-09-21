use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

#[derive(Debug)]
pub enum PointLightError {
    PositionMustBePoint
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Result<PointLight, PointLightError> {
        if position.is_vector() {
            return Err(PointLightError::PositionMustBePoint);
        }

        Ok(PointLight { position, intensity })
    }

    pub fn from_cords(x: f64, y: f64, z: f64, intensity: Color) -> PointLight {
        PointLight::new(Tuple::point(x, y, z), intensity).unwrap()
    }

    pub fn position(&self) -> Tuple {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::tuple::Tuple;

    #[test]
    fn point_in_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity).unwrap();

        assert_eq!(intensity, light.intensity);
        assert_eq!(position, light.position);
    }
}