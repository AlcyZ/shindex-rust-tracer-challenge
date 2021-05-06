use crate::math::f64_eq;
use crate::math::tuple::Tuple;
use crate::pattern::Pattern;
use crate::primitives::shape::Shape;
use crate::scene::shading::color::Color;
use crate::scene::shading::light::PointLight;

#[derive(Debug)]
pub(crate) struct Material {
    color: Color,
    pattern: Option<Box<dyn Pattern>>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
}

impl Material {
    pub(crate) fn new() -> Material {
        Material {
            color: Color::new(1., 1., 1.),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    pub(crate) fn lighting(
        &self,
        object: &dyn Shape,
        light: PointLight,
        position: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            Some(pattern) => pattern.pattern_at_shape(object, position),
            None => self.color,
        };

        // combine the surface color with the light's color/intensity
        let effective_color = color * light.intensity;

        // find the direction to the light source
        let light_v = (light.position - position).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        let mut diffuse = Color::new(0., 0., 0.);
        let mut specular = Color::new(0., 0., 0.);

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = light_v.dot(normal_v);
        if !in_shadow && light_dot_normal >= 0. {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye > 0. {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }

    pub(crate) fn get_color(&self) -> Color {
        self.color
    }

    pub(crate) fn set_color(&mut self, new: Color) {
        self.color = new
    }

    pub(crate) fn _get_ambient(&self) -> f64 {
        self.ambient
    }

    pub(crate) fn set_ambient(&mut self, new: f64) {
        self.ambient = new
    }

    pub(crate) fn _get_diffuse(&self) -> f64 {
        self.diffuse
    }

    pub(crate) fn set_diffuse(&mut self, new: f64) {
        self.diffuse = new
    }

    pub(crate) fn _get_specular(&self) -> f64 {
        self.specular
    }

    pub(crate) fn set_specular(&mut self, new: f64) {
        self.specular = new
    }

    pub(crate) fn _get_shininess(&self) -> f64 {
        self.shininess
    }

    pub(crate) fn _set_shininess(&mut self, new: f64) {
        self.shininess = new
    }

    pub(crate) fn get_transparency(&self) -> f64 {
        self.transparency
    }

    pub(crate) fn set_transparency(&mut self, new: f64) {
        self.transparency = new
    }

    pub(crate) fn get_refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub(crate) fn set_refractive_index(&mut self, new: f64) {
        self.refractive_index = new
    }

    pub(crate) fn get_reflective(&self) -> f64 {
        self.reflective
    }

    pub(crate) fn set_reflective(&mut self, new: f64) {
        self.reflective = new
    }

    pub(crate) fn set_pattern(&mut self, new: Box<dyn Pattern>) {
        self.pattern = Some(new)
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && f64_eq(self.ambient, other.ambient)
            && f64_eq(self.diffuse, other.diffuse)
            && f64_eq(self.specular, other.specular)
            && f64_eq(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::tuple::Tuple;
    use crate::pattern::stripe::StripePattern;
    use crate::primitives::sphere::Sphere;
    use crate::scene::shading::color::Color;
    use crate::scene::shading::light::PointLight;

    #[test]
    fn test_default_material() {
        let m = Material::new();

        assert_eq!(Color::new(1., 1., 1.), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
    }

    #[test]
    fn test_light_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = false;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., 0., -1.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(1.9, 1.9, 1.9), r);
    }

    #[test]
    fn test_light_with_eye_between_light_and_surface_with_45_degree_offset() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = false;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(1.0, 1.0, 1.0), r);
    }

    #[test]
    fn test_light_with_eye_opposite_surface_and_light_with_45_degree_offset() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = false;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., 0., -1.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), r);
    }

    #[test]
    fn test_light_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = false;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(1.636396, 1.636396, 1.636396), r);
    }

    #[test]
    fn test_light_behind_surface() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = false;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., 0., -1.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(0.1, 0.1, 0.1), r);
    }

    #[test]
    fn test_lighting_with_surface_in_shadow() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let in_shadow = true;
        let object = Sphere::new();

        let eye_v = Tuple::direction(0., 0., -1.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let r = m.lighting(&object, light, position, eye_v, normal_v, in_shadow);
        assert_eq!(Color::new(0.1, 0.1, 0.1), r);
    }

    #[test]
    fn test_lighting_with_pattern_applied() {
        let object = Sphere::new();
        let mut m = Material::new();
        m.set_pattern(Box::new(StripePattern::new(
            Color::new(1., 1., 1.),
            Color::new(0., 0., 0.),
        )));
        m.set_ambient(1.);
        m.set_diffuse(0.);
        m.set_specular(0.);

        let eye_v = Tuple::direction(0., 0., -1.);
        let normal_v = Tuple::direction(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));

        let c1 = m.lighting(
            &object,
            light,
            Tuple::point(0.9, 0., 0.),
            eye_v,
            normal_v,
            false,
        );
        let c2 = m.lighting(
            &object,
            light,
            Tuple::point(1.1, 0., 0.),
            eye_v,
            normal_v,
            false,
        );

        assert_eq!(Color::new(1., 1., 1.), c1);
        assert_eq!(Color::new(0., 0., 0.), c2);
    }

    #[test]
    fn test_reflectivity_of_default_material() {
        let m = Material::new();

        assert_eq!(0., m.reflective);
    }

    #[test]
    fn test_transparency_and_refractive_index_for_default_material() {
        let m = Material::new();

        assert_eq!(0.0, m.transparency);
        assert_eq!(1.0, m.refractive_index);
    }
}
