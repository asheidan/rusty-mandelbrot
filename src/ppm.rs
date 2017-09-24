// Taken from http://rosettacode.org/wiki/Bitmap/Write_a_PPM_file#Rust
use std::fs::File;
use std::io::Write;
use std::io::Result;

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PartialEq for RGB {
    fn eq(&self, other: &RGB) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b
    }
}

pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> PPM {
        let size = width * height * 3;
        let buffer = vec![0; size as usize];
        return PPM { width: width, height: height, data: buffer };
    }

    pub fn buffer_size(&self) -> u32 {
        return 3 * self.width * self.height;
    }

    pub fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width + x) * 3;
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }

    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let red = self.data[offset];
                let green = self.data[offset + 1];
                let blue = self.data[offset + 2];
                Some(RGB { r: red, g: green, b: blue })
            },
            None => None,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            },
            None => false,
        }
    }

    pub fn write_file(&self, file: &mut File) -> Result<()> {
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.data));
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_should_return_ppm_same_size() {
        // Given
        // When
        let ppm = PPM::new(4, 2);

        // Then
        assert_eq!(4, ppm.width);
        assert_eq!(2, ppm.height);
    }

    #[test]
    fn test_buffer_size_should_return_memory_needed_for_image() {
        // Given
        let ppm = PPM::new(10, 40);
        // When

        assert_eq!(10 * 40 * 3, ppm.buffer_size());
    }

    mod get_offset {
        use super::*;

        #[test]
        fn should_return_none_if_outside_of_buffer() {
            // Given
            let ppm = PPM::new(10, 10);
            // When
            let result = ppm.get_offset(10, 11);

            // Then
            assert_eq!(None, result);
        }

        #[test]
        fn should_return_some_data_if_in_buffer() {
            // Given
            let ppm = PPM::new(10, 10);

            // When
            let result = ppm.get_offset(5, 5);

            // Then
            assert!(result.is_some());
        }

        #[test]
        fn test_get_offset_should_return_correct_offset_if_in_buffer() {
            // Given
            let ppm = PPM::new(10, 10);

            // When
            let result = ppm.get_offset(5, 5).unwrap();

            // Then
            let expected = (5 * 10 + 5) * 3;

            assert_eq!(expected, result);
        }
    }
    mod get_pixel {
        use super::*;

        #[test]
        fn should_return_none_outside_of_picture() {
            // Given
            let ppm = PPM::new(1, 1);

            // When
            let result = ppm.get_pixel(2, 1);

            // Then
            assert!(result.is_none());
        }

        #[test]
        fn should_return_some_inside_of_picture() {
            // Given
            let ppm = PPM::new(1, 1);

            // When
            let result = ppm.get_pixel(0, 0);

            // Then
            assert!(result.is_some());
        }
    }
    mod set_pixel {
        use super::*;

        #[test]
        fn should_return_false_when_setting_pixel_outside() {
            // Given
            let mut ppm = PPM::new(1, 1);

            // When
            let result = ppm.set_pixel(1, 0, RGB { r: 0, g: 0, b: 0 });

            // Then
            assert_eq!(false, result);
        }

        #[test]
        fn should_return_true_when_setting_pixel_inside() {
            // Given
            let mut ppm = PPM::new(1, 1);

            // When
            let result = ppm.set_pixel(0, 0, RGB { r: 0, g: 0, b: 0 });

            // Then
            assert!(result);
        }
    }
}
