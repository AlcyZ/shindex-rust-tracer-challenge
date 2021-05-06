use crate::math::f64_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64,
}

impl Color {
    pub(crate) fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub(crate) fn white() -> Color {
        Color::new(1., 1., 1.)
    }

    pub(crate) fn black() -> Color {
        Color::new(0., 0., 0.)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_eq(self.red, other.red)
            && f64_eq(self.green, other.green)
            && f64_eq(self.blue, other.blue)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;

        Color::new(red, green, blue)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red - rhs.red;
        let green = self.green - rhs.green;
        let blue = self.blue - rhs.blue;

        Color::new(red, green, blue)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;

        Color::new(red, green, blue)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let red = self.red * rhs;
        let green = self.green * rhs;
        let blue = self.blue * rhs;

        Color::new(red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_are_like_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let e = Color::new(1.6, 0.7, 1.0);
        let r = c1 + c2;

        assert_eq!(e, r);
    }

    #[test]
    fn test_subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let e = Color::new(0.2, 0.5, 0.5);
        let r = c1 - c2;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_colors() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        let e = Color::new(0.9, 0.2, 0.04);
        let r = c1 * c2;

        assert_eq!(e, r);
    }

    #[test]
    fn test_multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        let e = Color::new(0.4, 0.6, 0.8);
        let r = c * 2f64;

        assert_eq!(e, r);
    }
}
