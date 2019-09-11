use crate::canvas::Canvas;
use std::fs::File;
use std::io::Write;

pub fn save_ppm(canvas: Canvas) {
    let ppm = canvas.to_ppm();
    let mut f = File::create("./PuttingItTogether/circle.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
