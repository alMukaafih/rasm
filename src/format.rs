//! This module defines target Formats for the generator.
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use jpeg_encoder as jpg;

use crate::image::*;

/// Image Format.
pub trait Format {
    /// Writes the Image to disk.
    fn write(&mut self, filename: &str);
    /// Retrieves the [Image] data of the format.
    fn image(&mut self) -> &mut Image;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Png Format.
pub struct Png {
    image: Image,
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
}
impl Png {
    /// Creates a new Png Image.
    pub fn new(width: usize, height: usize,) -> Png {
        Png {
            image: Image::new(width, height),
            color_type: png::ColorType::Rgba,
            bit_depth: png::BitDepth::Eight,
        }
    }
    /// Sets the Color Type of the Image.
    pub fn set_color_type(&mut self, color_type: png::ColorType) {
        self.color_type = color_type
    }
    /// Sets the Bit Depth of the Image.
    pub fn set_bit_depth(&mut self, bit_depth: png::BitDepth) {
        self.bit_depth = bit_depth
    }
}
impl Format for Png {
    fn write(&mut self, filename: &str) {
        let pth = format!("{filename}.png");
        let path = Path::new(&pth);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(
            w,
            self.image.width().try_into().unwrap(),
            self.image.height().try_into().unwrap(),
        );
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        let mut writer = encoder.write_header().unwrap();
        writer
            .write_image_data(self.image.to_vec().as_slice())
            .unwrap();
        //println!("{:?}", self.image[0][546][0]);
    }
    fn image(&mut self) -> &mut Image {
        &mut self.image
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Jpg Format.
pub struct Jpg {
    image: Image,
    color_type: jpg::ColorType,
}
impl Jpg {
    /// Creates a new Jpg Image.
    pub fn new(width: usize, height: usize) -> Jpg {
        Jpg {
            image: Image::new(width, height),
            color_type: jpg::ColorType::Rgba,
        }
    }
    /// Sets the Color Type of Image.
    pub fn set_color_type(&mut self, color_type: jpg::ColorType) {
        self.color_type = color_type
    }
}
impl Format for Jpg {
    fn write(&mut self, filename: &str) {
        let pth = format!("{filename}.jpg");
        // Create new encoder that writes to a file with maximum quality (100)
        let encoder = jpg::Encoder::new_file(pth, 100).unwrap();

        // Encode the data with dimension 2x2
        let _ = encoder.encode(
            self.image.to_vec().as_slice(),
            self.image.width.try_into().unwrap(),
            self.image.height.try_into().unwrap(),
            self.color_type,
        );
    }
    fn image(&mut self) -> &mut Image {
        &mut self.image
    }
}
