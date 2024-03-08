use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::*;

use crate::image::*;

/// Image Format.
pub trait Format {
    fn write(&mut self, filename: &str);
    fn image(&mut self) -> &mut Image;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Png Format.
pub struct Png {
    image: Image,
    color_type: ColorType,
    bit_depth: BitDepth
}
impl Png {
    pub fn new(width: usize, height: usize) -> Png {
        Png { 
            image: Image::with_dimensions(
                width, height
            ),
            color_type: ColorType::Rgba,
            bit_depth: BitDepth::Eight
        }
    }
    pub fn set_color_type(&mut self, color_type: ColorType) {
        self.color_type = color_type
    }
    pub fn set_bit_depth(&mut self, bit_depth: BitDepth) {
        self.bit_depth = bit_depth
    }
}
impl Format for Png {
    fn write(&mut self, filename: &str) {
        let pth = format!("{filename}.png");
        let path = Path::new(&pth);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = Encoder::new(
            w,
            self.image.width().try_into().unwrap(),
            self.image.height().try_into().unwrap()
        );
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(self.image.to_vec().as_slice()).unwrap();
        println!("{:?}", self.image[0][546][0]);
    }
    fn image(&mut self) -> &mut Image {
        &mut self.image
    }
}