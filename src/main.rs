use std::path::Path;
use std::fs::File;

mod palette;
mod ppm;

use ppm::PPM;
use palette::{ palette, PALETTE_SIZE };

const MAX_ITERATIONS: u32 = 2000;
const THRESHOLD: f64 = 2.0;

const MIN_X: f64 = -2.0;
const MIN_Y: f64 = -1.0;
const MAX_X: f64 = 1.0;
const MAX_Y: f64 = 1.0;

const RES_X: u32 = 2560 * 2;
const RES_Y: u32 = 1440 * 2;
//const RES_X: u32 = 300;
//const RES_Y: u32 = 200;

fn iterate(real: f64, imaginary: f64) -> u32 {
    let mut iterations: u32 = 0;
    let mut zr = 0.0;
    let mut zi = 0.0;

    while iterations < MAX_ITERATIONS && (zr * zr - zi * zi) <= THRESHOLD {
        let tmp = zr * zr - zi * zi + real;
        zi = 2.0 * zr * zi + imaginary;
        zr = tmp;

        iterations += 1;
    }

    return if iterations == MAX_ITERATIONS { u32::max_value() } else { iterations * PALETTE_SIZE / MAX_ITERATIONS };
}

fn main() {
    let mut image = PPM::new(RES_X, RES_Y);

    let step_x = (MAX_X - MIN_X) / (RES_X as f64);
    let step_y = (MAX_Y - MIN_Y) / (RES_Y as f64);

    println!("Rendering image");
    for y in 0..RES_Y {
        let imaginary_coord = MIN_Y + (y as f64) * step_y;

        for x in 0..RES_X {
            let real_coord = MIN_X + (x as f64) * step_x;

            let color = palette(iterate(real_coord, imaginary_coord));
            image.set_pixel(x, y, color);
        }
    }

    println!("Saving image");

    let path = Path::new(r"output.ppm");
    let mut file = File::create(path).unwrap();

    image.write_file(&mut file).unwrap();
}
