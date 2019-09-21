use std::fs::File;
use std::io::{Write, BufWriter};

use crate::canvas::Canvas;
use std::path::PathBuf;

pub fn save_ppm(canvas: Canvas, name: &str) {
    let ppm = canvas.to_ppm();
    let path = format!("./PuttingItTogether/{}.ppm", name);

    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}


pub fn save_png(canvas: Canvas, name: &str) {
    let file_name = format!("{}.png", name);
    let path: PathBuf = ["PuttingItTogether", &file_name].iter().collect();

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, canvas.width as u32, canvas.height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![];
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let color = canvas.pixel_at(x, y);
            data.push(col_to_u8(color.red));
            data.push(col_to_u8(color.green));
            data.push(col_to_u8(color.blue));
        }
    }

//    println!("data:\n{:#?}", data);

//    let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(data.as_slice()).unwrap();
}


fn col_to_u8(color: f64) -> u8 {
    let col: f64;
    if color > 1_f64 {
        col = 1_f64
    } else if color < 0_f64 {
        col = 0_f64
    } else {
        col = color
    }

    (col * 255__f64).round() as u8
}
