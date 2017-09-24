extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;

const PALETTE_SIZE: u32 = 256;
const MAX_ITERATIONS: u32 = 2000;
const THRESHOLD: f64 = 2.0;

const MIN_X: f64 = -2.0;
const MIN_Y: f64 = -1.0;
const MAX_X: f64 = 1.0;
const MAX_Y: f64 = 1.0;

//const RES_X: u32 = 2560;
//const RES_Y: u32 = 1440;
const RES_X: u32 = 640;
const RES_Y: u32 = 480;

fn iterate(real: f64, imaginary: f64) -> u8 {
    let mut iterations: u32 = 0;
    let mut zr = 0.0;
    let mut zi = 0.0;

    while iterations < MAX_ITERATIONS && (zr * zr - zi * zi) <= THRESHOLD {
        let tmp = zr * zr - zi * zi + real;
        zi = 2.0 * zr * zi + imaginary;
        zr = tmp;

        iterations += 1;
    }

    return if iterations == MAX_ITERATIONS { 0 } else { (iterations * PALETTE_SIZE / MAX_ITERATIONS) as u8 };
}

fn main() {
    let mut image_data: Vec<u8> = vec![0; ((RES_X * RES_Y) as usize)];

    let step_x = (MAX_X - MIN_X) / (RES_X as f64);
    let step_y = (MAX_Y - MIN_Y) / (RES_Y as f64);

    println!("Rendering image");
    for y in 0..RES_Y {
        let imaginary_coord = MIN_Y + (y as f64) * step_y;

        for x in 0..RES_X {
            let real_coord = MIN_X + (x as f64) * step_x;

            image_data.push(iterate(real_coord, imaginary_coord))
        }
    }

    println!("Saving image");

    let path = Path::new(r"output.png");
    let file = File::create(path).unwrap();
    let ref mut buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, RES_X, RES_Y);
    encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(image_data.as_ref()).unwrap();
}
