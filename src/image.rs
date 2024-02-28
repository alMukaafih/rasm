use std::ops::{Index, IndexMut};
use std::convert::From;
use std::fmt;
use std::iter::{Iterator, IntoIterator};
use crate::shape::*;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
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
        Pixel { red: 0, green: 0, blue: 0, alpha: 0, count: 0 }
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
    /// Alpha Composes the pixel using given [Pixel].
    pub fn compose(&mut self, pixel: Pixel) {
        let alpha = pixel[3];
        self.red = ((alpha * pixel[0]) + ((255 - alpha) * self.red)).div_ceil(255);
        self.green = ((alpha * pixel[1]) + ((255 - alpha) * self.green)).div_ceil(255);
        self.blue = ((alpha * pixel[2]) + ((255 - alpha) * self.blue)).div_ceil(255);
        self.alpha = (alpha + ((255 - alpha) * self.alpha)).div_ceil(255);
    }
}
impl From<&[u8]> for Pixel {
    fn from(pixel: &[u8]) -> Self {
        Pixel { red: pixel[0], green: pixel[1], blue: pixel[2], alpha: pixel[3], count: 0 }
    }
}
impl From<&[u8; 4]> for Pixel {
    fn from(pixel: &[u8; 4]) -> Self {
        Pixel { red: pixel[0], green: pixel[1], blue: pixel[2], alpha: pixel[3], count: 0 }
    }
}
impl Index<usize> for Pixel {
    type Output = u8;
    
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.red,
            1 => &self.blue,
            2 => &self.green,
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
            1 => &mut self.blue,
            2 => &mut self.green,
            3 => &mut self.alpha,
            4_usize.. => panic!("index out of bounds: the len is 4 but the index is {}", idx)
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
    length: usize,
    data: Vec<Pixel>
}
impl Row {
    /// Creates a new [Row] instance.
    pub fn new() -> Row {
        Row { length: 0, data: vec![] }
    }
    /// Creates a [Row] with length.
    pub fn with_length(length: usize) -> Row {
        Row { length: length, data: vec![Pixel::new(); length] }
    }
    /// Adds a [Pixel] to [Row].
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.data.push(pixel)
    }
    /// Retrieves a [Pixel] from [Row].
    pub fn get_pixel(&mut self, index: usize) -> &mut Pixel {
        &mut self.data[index - 1]
    }
    pub fn length(&self) -> usize {
        self.length
    }
}
impl From<&[u8]> for Row {
    fn from(data: &[u8]) -> Row {
        let mut row = Row::new();
        for i in data.chunks_exact(4) {
            row.add_pixel(Pixel::from(i))
        }
        row.length = data.len();
        row
    }
}
impl Index<usize> for Row {
    type Output = Pixel;
    
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
impl IndexMut<usize> for Row {
    //type Output = Pixel;
    
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
    
}
impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Row ");
        f.debug_list().entries(self.data.iter()).finish()
    }
}
impl IntoIterator for Row {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// Layer in an Image representation.
pub struct Layer {
    width: usize,
    height: usize,
    data: Vec<Row>
}
impl Layer {
    /// Creates a new [Layer] instance.
    pub fn new() -> Layer {
        Layer { width: 0, height: 0, data: vec![] }
    }
    /// Creates a [Layer] with dimensions.
    pub fn with_dimensions(width: usize, height: usize) -> Layer {
        Layer { width: width, height: height, data: vec![Row::with_length(width); height] }
    }
    /// Adds a [Row] to [Layer].
    pub fn add_row(&mut self, row: Row) {
        self.data.push(row)
    }
    /// Retrives a [Row] from [Layer].
    pub fn get_row(&mut self, index: usize) -> &mut Row {
        &mut self.data[index - 1]
    }
    pub fn mut_row(&mut self, idx: usize, color: Pixel) {
        for i in 0..self.width {
            self[idx][i] = color;
        }
    }
    pub fn mut_col(&mut self, idx: usize, color: Pixel) {
        for i in 0..self.height {
            self[i][idx] = color;
        }
    }
    pub fn fill(&mut self, color: Pixel) {
        for rows in 0..self.height {
            for pixels in 0..self.width {
                self[rows][pixels] = color;
            }
        }
    }
    pub fn construct(_rect: Rectangle, width: usize, _height: usize) -> Layer {
        let bytes: &[u8] = &[0];
        Layer::from((width, bytes))
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
impl From<(usize, usize, &[u8])> for Layer {
    fn from(image: (usize, usize, &[u8])) -> Self {
        let mut layer = Layer::new();
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
        let mut layer = Layer::new();
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
        &self.data[idx]
    }
}
impl IndexMut<usize> for Layer {
    //type Output = Pixel;
    
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
    
}
impl fmt::Debug for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Layer ");
        f.debug_list().entries(self.data.iter()).finish()
    }
}
impl IntoIterator for Layer {
    type Item = Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// Image representation.
pub struct Image {
    width: usize,
    height: usize,
    layers: usize,
    data: Vec<Layer>
}
impl Image {
    /// Creates a new [Image] instance.
    pub fn new() -> Image {
        Image { width: 0, height: 0, layers: 0, data: vec![] }
    }
    /// Creates an [Image] with given dimensions.
    pub fn with_dimensions(width: usize, height: usize) -> Image {
        Image {
            width: width,
            height: height,
            layers: 1,
            data: vec![Layer::with_dimensions(width, height)]
        }
    }
    /// Collapses all layers to a single layer.
    pub fn collapse(&mut self) {
        let mut img = Image::with_dimensions(self.width, self.height);
        for layers in 0..self.layers {
        for rows in 0..self[layers].height {
        for pixels in 0..self[layers][rows].length {
            let mut pix = img[0][rows][pixels];
            let pixel = self[layers][rows][pixels];
            if pixel[3] == 0 {
                continue
            }
            if pix[3] == 0 {
                img[0][rows][pixels] = pixel;
                continue
            }
            pix.compose(pixel);
        }
        }
        }
        *self = img;
        
    }
    /// Adds a [Layer] to [Image].
    pub fn add_layer(&mut self, layer: Layer) {
        self.data.push(layer);
        self.layers += 1
    }
    /// Retrieves a [Layer] from [Image].
    pub fn get_layer(&mut self, index: usize) -> &mut Layer {
        &mut self.data[index - 1]
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn to_vec(&mut self) -> Vec<u8> {
        self.collapse();
        let mut bytes: Vec<u8> = Vec::new();
        for rows in 0..self[0].height {
            for pixels in 0..self[0][rows].length {
                bytes.push(self[0][rows][pixels][0]);
                bytes.push(self[0][rows][pixels][1]);
                bytes.push(self[0][rows][pixels][2]);
                bytes.push(self[0][rows][pixels][3])
            }
        }
        bytes
    }
}
impl From<(usize, usize, &[u8])> for Image {
    fn from(image: (usize, usize, &[u8])) -> Self {
        let mut img = Image::new();
        img[0] = Layer::from(image);
        img
    }
}
impl From<(usize, &[u8])> for Image {
    fn from(image: (usize, &[u8])) -> Self {
        let mut img = Image::new();
        img[0] = Layer::from(image);
        img
    }
}
impl Index<usize> for Image {
    type Output = Layer;
    
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
impl IndexMut<usize> for Image {
    //type Output = Pixel;
    
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
    
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("Image ");
        f.debug_list().entries(self.data.iter()).finish()
    }
}
impl IntoIterator for Image {
    type Item = Layer;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

