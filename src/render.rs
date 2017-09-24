use palette::PALETTE_SIZE;

const MAX_ITERATIONS: u32 = 1500;
const THRESHOLD: f64 = 2.0;

pub fn iterate(real: f64, imaginary: f64) -> u32 {
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
