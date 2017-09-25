use std::path::Path;
use std::fs::File;

mod palette;
mod ppm;
mod render;

use ppm::PPM;
use palette::palette;
use render::iterate;

const CEN_REAL: f64 = -0.7345;
const CEN_IMAG: f64 = 0.1955;
const INT_REAL: f64 = 0.0015;
const INT_IMAG: f64 = 0.001;

const MIN_X: f64 = CEN_REAL - (INT_REAL / 2.0);  // -2.0;
const MIN_Y: f64 = CEN_IMAG - (INT_IMAG / 2.0);  // -1.0;
const MAX_X: f64 = CEN_REAL + (INT_REAL / 2.0);  // 1.0;
const MAX_Y: f64 = CEN_IMAG + (INT_IMAG / 2.0);  // 1.0;

const RES_X: u32 = 2560 * 2;
const RES_Y: u32 = 1440 * 2;
//const RES_X: u32 = 300;
//const RES_Y: u32 = 200;


fn main() {
    let mut image = PPM::new(RES_X, RES_Y);

    let step_x = (MAX_X - MIN_X) / (RES_X as f64);
    let step_y = (MAX_Y - MIN_Y) / (RES_Y as f64);

    println!("Rendering image");
    for y in 0..RES_Y {
        // Going top to bottom
        let imaginary_coord = MAX_Y - (y as f64) * step_y;

        for x in 0..RES_X {
            // Going left to right
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
