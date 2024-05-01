//! This module defines struct for working with Image data.
use std::convert::From;
use std::fmt;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Add, AddAssign};
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};
use std::fs::read;
use std::path::*;

use file_format::FileFormat;
use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;

use crate::object::*;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
/// Pixel in a Row representation.
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    count: usize,
}
impl Pixel {
    /// Creates a new [Pixel] instance.
    pub fn new() -> Pixel {
        Pixel {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
            count: 255,
        }
    }
    /// Sets red value of [Pixel].
    pub fn set_red(&mut self, red: u8) -> &mut Pixel {
        self.red = red;
        self
    }
    /// Sets blue value of [Pixel].
    pub fn set_blue(&mut self, blue: u8) -> &mut Pixel {
        self.blue = blue;
        self
    }
    /// Sets green value of [Pixel].
    pub fn set_green(&mut self, green: u8) -> &mut Pixel {
        self.green = green;
        self
    }
    /// Sets alpha value of [Pixel].
    pub fn set_alpha(&mut self, alpha: u8) -> &mut Pixel {
        self.alpha = alpha;
        self
    }
}
impl Default for Pixel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<&[u8]> for Pixel {
    fn from(pixel: &[u8]) -> Self {
        Pixel {
            red: pixel[0],
            green: pixel[1],
            blue: pixel[2],
            alpha: pixel[3],
            count: 0,
        }
    }
}
impl From<&[u8; 4]> for Pixel {
    fn from(pixel: &[u8; 4]) -> Self {
        Pixel {
            red: pixel[0],
            green: pixel[1],
            blue: pixel[2],
            alpha: pixel[3],
            count: 0,
        }
    }
}
impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            3 => &self.alpha,
            4_usize.. => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}
impl IndexMut<usize> for Pixel {
    //type Output = Pixel;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            3 => &mut self.alpha,
            4_usize.. => panic!("index out of bounds: the len is 4 but the index is {}", idx),
        }
    }
}
impl Add for Pixel {
    type Output = Self;

    fn add(self, pixel: Self) -> Self {
        let alpha = pixel[3] as u64;
        Self {
            red: ((alpha * pixel[0] as u64) + ((255 - alpha) * self.red as u64)).div_ceil(255)
                as u8,
            green: ((alpha * pixel[1] as u64) + ((255 - alpha) * self.green as u64)).div_ceil(255)
                as u8,
            blue: ((alpha * pixel[2] as u64) + ((255 - alpha) * self.blue as u64)).div_ceil(255)
                as u8,
            alpha: (alpha * 255 + ((255 - alpha) * self.alpha as u64)).div_ceil(255) as u8,
            count: 0,
        }
    }
}
impl AddAssign for Pixel {
    fn add_assign(&mut self, pixel: Self) {
        let alpha = pixel[3] as u64;
        *self = Self {
            red: ((alpha * pixel[0] as u64) + ((255 - alpha) * self.red as u64)).div_ceil(255)
                as u8,
            green: ((alpha * pixel[1] as u64) + ((255 - alpha) * self.green as u64)).div_ceil(255)
                as u8,
            blue: ((alpha * pixel[2] as u64) + ((255 - alpha) * self.blue as u64)).div_ceil(255)
                as u8,
            alpha: (alpha * 255 + ((255 - alpha) * self.alpha as u64)).div_ceil(255) as u8,
            count: 0,
        }
    }
}
impl Iterator for Pixel {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        if self.count == 1 {
            Some(self.red)
        } else if self.count == 2 {
            Some(self.green)
        } else if self.count == 3 {
            Some(self.blue)
        } else if self.count == 4 {
            Some(self.alpha)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// Row in a Layer representation.
pub struct Row {
    pixels: Vec<Pixel>,
}
impl Row {
    /// Creates a new [Row] instance.
    pub fn new(length: usize) -> Row {
        Row {
            pixels: vec![Pixel::new(); length],
        }
    }
    /// Adds a [Pixel] to Row.
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels.push(pixel)
    }
    /// Returns a [Pixel] from [Row] at given index.
    pub fn get_pixel(&mut self, index: usize) -> &mut Pixel {
        &mut self.pixels[index - 1]
    }
}
impl Default for Row {
    fn default() -> Self {
        Self::new(1080)
    }
}
impl From<&[u8]> for Row {
    fn from(data: &[u8]) -> Row {
        let mut row = Row::new(0);
        for i in data.chunks_exact(4) {
            row.add_pixel(Pixel::from(i))
        }
        row
    }
}
impl Index<usize> for Row {
    type Output = Pixel;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.pixels[idx]
    }
}
impl IndexMut<usize> for Row {
    //type Output = Pixel;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.pixels[idx]
    }
}
impl Deref for Row {
    type Target = Vec<Pixel>;

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}
impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Row ");
        f.debug_list().entries(self.pixels.iter()).finish()
    }
}
impl IntoIterator for Row {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// Layer in an Image representation.
pub struct Layer {
    width: usize,
    height: usize,
    rows: Vec<Row>,
}
impl Layer {
    /// Creates a new [Layer] instance.
    pub fn new(width: usize, height: usize) -> Layer {
        Layer {
            width,
            height,
            rows: vec![Row::new(width); height],
        }
    }
    /// Adds a [Row] to Layer.
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row)
    }
    /// Returns a [Row] from Layer at given index.
    pub fn get_row(&mut self, index: usize) -> &mut Row {
        &mut self.rows[index - 1]
    }
    /// Mutates a Row.
    pub fn mut_row(&mut self, idx: usize, color: Pixel) {
        for i in 0..self.width {
            self[idx][i] = color;
        }
    }
    /// Mutates a Column.
    pub fn mut_col(&mut self, idx: usize, color: Pixel) {
        for i in 0..self.height {
            self[i][idx] = color;
        }
    }
    /// Fills the Image with given Color.
    pub fn fill(&mut self, color: Pixel) {
        for rows in 0..self.height {
            for pixels in 0..self.width {
                self[rows][pixels] = color;
            }
        }
    }
    /// Constructs a [Rectangle][R]
    ///
    /// [R]: Rect
    pub fn construct(_rect: Rect, width: usize, _height: usize) -> Layer {
        let bytes: &[u8] = &[0];
        Layer::from((width, bytes))
    }
    /// Returns the width of Layer.
    pub fn width(&self) -> usize {
        self.width
    }
    /// Returns the height of Layer.
    pub fn height(&self) -> usize {
        self.height
    }
}
impl Default for Layer {
    fn default() -> Self {
        Self::new(1080, 1080)
    }
}
impl From<(usize, usize, Vec<u8>)> for Layer {
    fn from(image: (usize, usize, Vec<u8>)) -> Self {
        let mut layer = Layer::new(0,0);
        layer.width = image.0;
        layer.height = image.1;
        for i in image.2.chunks_exact(4 * image.0) {
            let row = Row::from(i);
            layer.add_row(row)
        }
        layer
    }
}
impl From<(usize, usize, &[u8])> for Layer {
    fn from(image: (usize, usize, &[u8])) -> Self {
        let mut layer = Layer::new(0,0);
        layer.width = image.0;
        layer.height = image.1;
        for i in image.2.chunks_exact(4 * image.0) {
            let row = Row::from(i);
            layer.add_row(row)
        }
        layer
    }
}
impl From<(usize, &[u8])> for Layer {
    fn from(image: (usize, &[u8])) -> Self {
        let mut layer = Layer::new(0,0);
        layer.width = image.0;
        let mut x = 0;
        for i in image.1.chunks_exact(4 * image.0) {
            let row = Row::from(i);
            layer.add_row(row);
            x += 1
        }
        layer.height = x;
        layer
    }
}
impl Index<usize> for Layer {
    type Output = Row;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.rows[idx]
    }
}
impl IndexMut<usize> for Layer {
    //type Output = Pixel;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.rows[idx]
    }
}
impl Deref for Layer {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}
impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Layer ");
        f.debug_list().entries(self.rows.iter()).finish()
    }
}
impl IntoIterator for Layer {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// Image representation.
pub struct Image {
    /// Width of Image.
    pub width: usize,
    /// Height of Image.
    pub height: usize,
    /// Origin of Image.
    pub origin: Point,
    /// Layers in Image.
    pub layers: Vec<Layer>,
}
impl Image {
    /// Creates a new [Image] instance.
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            origin: Point::from((0, 0)),
            layers: vec![Layer::new(width, height)],
        }
    }
    /// Collapses all layers to a single layer.
    pub fn collapse(&mut self) {
        if self.layers.len() == 1 {
            return;
        }
        let mut img = Image::new(self.width, self.height);
        for layers in 0..self.layers.len() {
            for rows in 0..self[layers].height {
                for pixels in 0..self[layers][rows].len() {
                    let pix = img[0][rows][pixels];
                    let pixel = self[layers][rows][pixels];
                    if pixel[3] == 0 {
                        continue;
                    }
                    if pix[3] == 0 {
                        img[0][rows][pixels] = pixel;
                        continue;
                    }
                    img[0][rows][pixels] = pix + pixel;
                }
            }
        }
        *self = img;
    }
    /// Adds a [Layer] to Image.
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    /// Returns a [Layer] from Image at given index.
    pub fn get_layer(&mut self, index: usize) -> &mut Layer {
        &mut self.layers[index - 1]
    }
    /// Returns the width of the Image.
    pub fn width(&self) -> usize {
        self.width
    }
    /// Returns the height of the Image.
    pub fn height(&self) -> usize {
        self.height
    }
    /// Converts the Image to a [`Vec<u8>`].
    pub fn to_vec(&mut self) -> Vec<u8> {
        if self.layers.len() > 1 {
            self.collapse();
        }
        let mut bytes: Vec<u8> = Vec::new();
        for rows in 0..self[0].height {
            for pixels in 0..self[0][rows].len() {
                bytes.push(self[0][rows][pixels][0]);
                bytes.push(self[0][rows][pixels][1]);
                bytes.push(self[0][rows][pixels][2]);
                bytes.push(self[0][rows][pixels][3])
            }
        }
        bytes
    }
    /// Converts the Image to a [`Vec<Pixel>`].
    pub fn as_pixels(&mut self) -> Vec<Pixel> {
        if self.layers.len() > 1 {
            self.collapse();
        }
        let mut bytes: Vec<Pixel> = Vec::new();
        for rows in 0..self[0].height {
            for pixels in 0..self[0][rows].len() {
                bytes.push(self[0][rows][pixels]);
            }
        }
        bytes
    }
    /// Creates an Image from file on disk.
    pub fn from_file<R: AsRef<Path>>(filename: R) -> Image {
        let fmt = FileFormat::from_file(&filename).unwrap();

        if fmt.media_type() == "image/png" {
            let file_contents = read(filename).unwrap();
            let options = DecoderOptions::default()
                .png_set_add_alpha_channel(true)
                .png_set_strip_to_8bit(true);
            // use the above option to decode
            let mut decoder = PngDecoder::new_with_options(&file_contents, options);
        
            let buf = decoder.decode_raw().unwrap();
            let info = decoder.get_info().unwrap();
            let width = info.width as usize;
            let height = info.height as usize;
            return Image::from(((width, height), (0,0), buf))
        } 
        else if fmt.media_type() == "image/jpeg" {
            let file_contents = read(filename).unwrap();
            let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);
        
            let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
            let buf = decoder.decode().unwrap();
            let info = decoder.info().unwrap();
            let width = info.width as usize;
            let height = info.height as usize;
            return Image::from(((width, height), (0,0), buf))
        } else {
            panic!("I know it's an Image file, but I don't know what to do with it.")
        }
    }
}
impl Default for Image {
    fn default() -> Self {
        Self::new(1080, 1080)
    }
}
impl From<((usize, usize), (usize, usize), Vec<u8>)> for Image {
    fn from(img: ((usize, usize), (usize, usize), Vec<u8>)) -> Self {
        Image {
            width: img.0 .0,
            height: img.0 .1,
            origin: Point::from(img.1),
            layers: vec![Layer::from((img.0 .0, img.0 .1, img.2))],
        }
    }
}
impl From<(usize, usize, &[u8])> for Image {
    fn from(image: (usize, usize, &[u8])) -> Self {
        let mut img = Image::new(0,0);
        img[0] = Layer::from(image);
        img
    }
}
impl From<(usize, &[u8])> for Image {
    fn from(image: (usize, &[u8])) -> Self {
        let mut img = Image::new(0,0);
        img[0] = Layer::from(image);
        img
    }
}
impl Index<usize> for Image {
    type Output = Layer;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.layers[idx]
    }
}
impl IndexMut<usize> for Image {
    //type Output = Pixel;

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.layers[idx]
    }
}
impl Deref for Image {
    type Target = Vec<Layer>;

    fn deref(&self) -> &Self::Target {
        &self.layers
    }
}
impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layers
    }
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Image ");
        f.debug_list().entries(self.layers.iter()).finish()
    }
}
impl IntoIterator for Image {
    type Item = Layer;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.into_iter()
    }
}
