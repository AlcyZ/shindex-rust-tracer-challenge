use crate::color::{Color, color};

#[derive(Debug)]
struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

fn canvas(width: usize, height: usize) -> Canvas {
    let mut pixels: Vec<Vec<Color>> = Vec::new();
    let black = color(0_f64, 0_f64, 0_f64);


    for w in 0..width {
        for h in 0..height {
            let mut foo: Vec<Color> = Vec::new();
            foo.push(black.clone());

            pixels.push(foo)
        }
    }

//    let r = vec![color(0_f64, 0_f64, 0_f64); width];
//    let pixels = vec![r; height];

    Canvas { width, height, pixels }
}

#[cfg(Test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
    }
}
