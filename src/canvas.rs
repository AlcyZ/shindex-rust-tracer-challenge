use crate::color::{col_to_string, Color};

pub struct Canvas {
    width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

fn ppm_push(data: &mut Vec<String>, line: &mut String, col_str: String) {
    if line.len() > 67 {
        data.push(line[0..line.len() - 1].to_string());
        line.truncate(0);
    }
    line.push_str(&col_str);
    line.push_str(" ");
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let r = vec![Color::black(); width];
        let pixels = vec![r; height];

        Canvas { width, height, pixels }
    }

    fn pixel_at(&self, width: usize, height: usize) -> &Color {
        &self.pixels[height][width]
    }

    pub fn write_pixel(&mut self, width: usize, height: usize, c: Color) {
        if let Option::Some(foo) = self.pixels.get(height) {
            if let Option::Some(_) = foo.get(width) {
                self.pixels[height][width] = c
            }
        }
    }

    pub fn to_ppm(&self) -> String {
        let header = self.ppm_header();
        let body = self.ppm_body();
        let data = [header, body];

        data.join("\n")
    }

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    fn ppm_body(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        for h in 0..self.height {
            let mut line = "".to_string();
            for w in 0..self.width {
                let pix = self.pixel_at(w, h);
                let red = col_to_string(&pix.red);
                let green = col_to_string(&pix.green);
                let blue = col_to_string(&pix.blue);


                ppm_push(&mut lines, &mut line, red);
                ppm_push(&mut lines, &mut line, green);
                ppm_push(&mut lines, &mut line, blue);
            }
            lines.push(line[0..line.len() - 1].to_string())
        }

        lines.push("".to_string());
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        let black = Color::new(0_f64, 0_f64, 0_f64);

        assert_eq!(10, c.width);
        assert_eq!(20, c.height);

        for w in 0..c.width {
            for h in 0..c.height {
                assert_eq!(&black, c.pixel_at(w, h));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1_f64, 0_f64, 0_f64);

        c.write_pixel(2, 3, red);
        assert_eq!(&Color::new(1_f64, 0_f64, 0_f64), c.pixel_at(2, 3))
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let expected = ["P3", "5 3", "255"];

        for (i, l) in ppm.lines().enumerate() {
            if i < 3 {
                assert_eq!(l, expected[i])
            }
        }
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0_f64, 0_f64);
        let c2 = Color::new(0_f64, 0.5, 0_f64);
        let c3 = Color::new(-0.5, 0_f64, 1_f64);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();
        let expected = [
            "P3",
            "5 3",
            "255",
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
        ];

        let mut asserted = false;
        for (i, l) in ppm.lines().enumerate() {
            if i >= 3 && i <= 5 {
                assert_eq!(l, expected[i]);
                asserted = true;
            }
        }
        assert!(asserted)
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        for w in 0..c.width {
            for h in 0..c.height {
                c.write_pixel(w, h, Color::new(1_f64, 0.8, 0.6))
            }
        }

        let expected = [
            "P3",
            "10 2",
            "255",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ];
        let ppm = c.to_ppm();

        let mut asserted = false;
        for (i, l) in ppm.lines().enumerate() {
            if i >= 3 && i <= 6 {
                assert_eq!(l, expected[i]);
                asserted = true;
            }
        }
        assert!(asserted)
    }
}
