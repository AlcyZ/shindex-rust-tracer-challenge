use crate::color::Color;
use crate::light::PointLight;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

#[derive(Debug)]
pub enum MaterialError {
    InvalidNegativeFloat
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Result<Material, MaterialError> {
        if ambient < 0.0 || diffuse < 0.0 || specular < 0.0 || shininess < 0.0 {
            return Err(MaterialError::InvalidNegativeFloat);
        }

        Ok(Material { color, ambient, diffuse, specular, shininess })
    }

    pub fn default() -> Material {
        Material::new(Color::white(), 0.1, 0.9, 0.9, 200.0).unwrap()
    }

    pub fn lighting(&self, light: PointLight, point: Tuple, eye_v: Tuple, normal_v: Tuple, in_shadow: bool) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity();

        // find the direction to the light source
        let light_v = (light.position() - point).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = light_v.dot(normal_v);
        if in_shadow {
            return ambient;
        }

        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // compute specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor
            }
        }
        // Add the three contributions together to get the final shading
        ambient + diffuse + specular
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn change_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn change_ambient(&mut self, ambient: f64) {
        self.ambient = ambient
    }

    pub fn change_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }

    pub fn change_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    pub fn change_shininess(&mut self, shininess: f64) {
        self.shininess = shininess
    }
}

impl std::cmp::PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color &&
            self.ambient == other.ambient &&
            self.diffuse == other.diffuse &&
            self.specular == other.specular &&
            self.shininess == other.shininess
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::tuple::Tuple;

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    fn setup() -> (Material, Tuple) {
        (Material::default(), Tuple::origin_point())
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = setup();
        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white()).unwrap();
        let in_shadow = false;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_45_deg_offset_between_light_and_surface() {
        let (m, position) = setup();
        let a = 2_f64.sqrt() / 2.0;
        let eye_v = Tuple::vector(0.0, a, -a);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white()).unwrap();
        let in_shadow = false;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_46_deg() {
        let (m, position) = setup();
        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white()).unwrap();
        let in_shadow = false;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = setup();
        let a = 2_f64.sqrt() / 2.0;
        let eye_v = Tuple::vector(0.0, -a, -a);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white()).unwrap();
        let in_shadow = false;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white()).unwrap();
        let in_shadow = false;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let (m, position) = setup();
        let eye_v = Tuple::vector(0.0, 0.0, -1.0);
        let normal_v = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white()).unwrap();
        let in_shadow = true;

        let result = m.lighting(light, position, eye_v, normal_v, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}