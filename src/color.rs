#[derive(Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl std::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color { red, green, blue }
}

#[cfg(test)]
mod tests {
    use crate::color::*;

    #[test]
    fn color_are_red_green_blue() {
        let c = color(1_f64, 2_f64, 3_f64);

        assert_eq!(c.red, 1_f64);
        assert_eq!(c.green, 2_f64);
        assert_eq!(c.blue, 3_f64);
    }

    #[test]
    fn adding_colors() {
        let a = color(1_f64, 2_f64, 3_f64);
        let b = color(2_f64, 3_f64, 4_f64);
        let c = color(3_f64, 5_f64, 7_f64);

        assert_eq!(c, a + b)
    }

    #[test]
    fn subtracting_colors() {
        let a = color(1_f64, 2_f64, 3_f64);
        let b = color(2_f64, 3_f64, -4_f64);
        let c = color(-1_f64, -1_f64, 7_f64);

        assert_eq!(c, a - b)
    }

    #[test]
    fn multiply_color_and_scalar() {
        let a = color(1_f64, 2_f64, 3_f64);
        let b = color(3.5, 7_f64, 10.5);

        assert_eq!(b, a * 3.5)
    }

    #[test]
    fn multiply_colors() {
        let a = color(1_f64, 2_f64, 3_f64);
        let b = color(2_f64, 3_f64, 4_f64);
        let c = color(2_f64, 6_f64, 12_f64);

        assert_eq!(c, a * b)
    }
}
