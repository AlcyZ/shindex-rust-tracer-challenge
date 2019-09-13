use std::fs::File;
use std::io::Write;

use crate::canvas::Canvas;

pub fn save_ppm(canvas: Canvas, name: &str) {
    let ppm = canvas.to_ppm();
    let path = format!("./PuttingItTogether/{}.ppm", name);

    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
