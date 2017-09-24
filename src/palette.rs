use ppm::RGB;

pub const PALETTE_SIZE: u32 = 1275;

pub fn palette(i: u32) -> RGB {
    match i {
        0 ... 205 => {
            // Black to blue
            RGB { r: 0, g: 0, b: (50 + i) as u8}
        },
        205 ... 460 => {
            // Blue to white
            let color = (i - 205) as u8;
            RGB { r: color, g: color, b: 255}
        },
        460 ... 715 => {
            // White to yellow
            RGB { r: 255, g: 255, b: (715 - i) as u8 }
        },
        715 ... 970 => {
            // Yellow to red
            RGB { r: 255, g: (970 - i) as u8, b: 0 }
        },
        970 ... 1125 => {
            // Red to dark red
            RGB { r: (1225 - i) as u8, g: 0, b: 0 }
        },
        1125 ... 1175 => {
            // Dark red to purple
            RGB { r: 100, g: 0, b: (i - 1125) as u8 }
        },
        1175 ... 1275 => {
            // Purple to blue
            RGB { r: (1275 - i) as u8, g: 0, b: 50 as u8 }
        }
        _ => RGB { r: 0, g: 0, b: 0},
    }
}

#[cfg(test)]
mod tests_palette {
    use super::*;

    #[test]
    fn too_many_iterations_should_be_black() {
        // Given
        // When
        let result = palette(u32::max_value());
        // Then
        let expected = RGB { r: 0, g: 0, b: 0 };
        assert_eq!(expected, result);
    }
}
