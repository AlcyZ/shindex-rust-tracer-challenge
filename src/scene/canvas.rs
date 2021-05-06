use crate::scene::shading::color::Color;

#[derive(Debug)]
pub(crate) struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub(crate) fn new(width: usize, height: usize) -> Canvas {
        let row = vec![Color::new(0., 0., 0.); width];
        let pixels = vec![row; height];

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub(crate) fn pixel_at(&self, width: usize, height: usize) -> Option<Color> {
        if width > self.width || height > self.height {
            return None;
        }

        Some(self.pixels[height][width])
    }

    pub(crate) fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        if width > self.width || height > self.height {
            return;
        }

        self.pixels[height][width] = color;
    }

    pub(crate) fn to_ppm(&self) -> String {
        let mut header = format!("P3\n{} {}\n255", self.width, self.height);
        let mut data = String::new();

        for h in 0..self.height {
            let mut row = String::from("\n");

            for w in 0..self.width {
                let pixel = self.pixel_at(w, h).unwrap();

                let red = Canvas::color_byte_string(pixel.red);
                if row.len() + red.len() > 70 {
                    data.push_str(&row.trim_end());
                    row = String::from("\n");
                }
                row.push_str(&red);
                row.push_str(" ");

                let green = Canvas::color_byte_string(pixel.green);
                if row.len() + green.len() > 70 {
                    data.push_str(&row.trim_end());
                    row = String::from("\n");
                }
                row.push_str(&green);
                row.push_str(" ");

                let blue = Canvas::color_byte_string(pixel.blue);
                if row.len() + blue.len() > 70 {
                    data.push_str(&row.trim_end());
                    row = String::from("\n");
                }
                row.push_str(&blue);
                row.push_str(" ");
            }

            data.push_str(&row.trim_end());
        }

        header.push_str(&data);
        header.push_str("\n");

        header
    }

    fn color_byte_string(color: f64) -> String {
        (color.clamp(0., 1.) * 255.).round().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::shading::color::Color;

    #[test]
    fn test_creating_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        let e = Color::new(0., 0., 0.);

        for w in 0..10 {
            for h in 0..20 {
                let r = c.pixel_at(w, h);
                assert_eq!(e, r.unwrap());
            }
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        c.write_pixel(2, 3, red);

        assert_eq!(red, c.pixel_at(2, 3).unwrap())
    }

    #[test]
    fn test_create_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        let lines = ppm.split("\n").collect::<Vec<&str>>();

        assert_eq!("P3", lines[0]);
        assert_eq!("5 3", lines[1]);
        assert_eq!("255", lines[2]);
    }

    #[test]
    fn test_create_ppm_data() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<&str>>();

        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines[3]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines[4]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines[5]);
    }

    #[test]
    fn test_splitting_to_long_ppm_lines() {
        let mut c = Canvas::new(10, 2);

        for w in 0..10 {
            for h in 0..2 {
                c.write_pixel(w, h, Color::new(1., 0.8, 0.6));
            }
        }

        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<&str>>();

        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            lines[3]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            lines[4]
        );
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            lines[5]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            lines[6]
        );
    }

    #[test]
    fn test_ppm_ends_with_newlines() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.ends_with("\n"));
    }
}
