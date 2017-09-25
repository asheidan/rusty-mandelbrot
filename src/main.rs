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

const RES_X: u32 = 2560 / 2;
const RES_Y: u32 = 1440 / 2;
//const RES_X: u32 = 300;
//const RES_Y: u32 = 200;


fn stuff() {
    let mut image = PPM::new(RES_X, RES_Y);

    let step_x = (MAX_X - MIN_X) / (RES_X as f64);
    let step_y = (MAX_Y - MIN_Y) / (RES_Y as f64);

    println!("Rendering image");

    let progress_step = RES_Y / 20;

    for y in 0..RES_Y {
        if 0 == y % progress_step {
            println!("{}%", y * 100 / RES_Y);
        }
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

extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

// http://www.willusher.io/sdl2%20tutorials/2013/08/17/lesson-1-hello-world

fn main() {
    let mut data: Vec<u8> = vec![0; (3 * RES_X * RES_Y) as usize];

    let sdl_context = sdl2::init().unwrap();
    //let mut sdl_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem.window("Window", RES_X, RES_Y).build().unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
    canvas.clear();

    let surface = Surface::from_data(&mut data, RES_X, RES_Y, 3, PixelFormatEnum::RGB24);

    canvas.present();

    std::thread::sleep(std::time::Duration::from_secs(10));
}
