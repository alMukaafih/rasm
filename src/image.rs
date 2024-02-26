use std::ops::{Index, IndexMut};
use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}
impl Pixel {
    pub fn new() -> Pixel {
        Pixel { red: 0u8, green: 0u8, blue: 0u8, alpha: 0u8 }
    }
    pub fn from(pixel: &[u8]) -> Pixel {
        Pixel { red: pixel[0], green: pixel[1], blue: pixel[2], alpha: pixel[3] }
    }
    pub fn set_red(&mut self, red: u8) -> &mut Pixel {
        self.red = red;
        self
    }
    pub fn set_green(&mut self, green: u8) -> &mut Pixel {
        self.green = green;
        self
    }
    pub fn set_blue(&mut self, blue: u8) -> &mut Pixel {
        self.blue = blue;
        self
    }
    pub fn set_alpha(&mut self, alpha: u8) -> &mut Pixel {
        self.alpha = alpha;
        self
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Row {
    data: Vec<Pixel>
}
impl Row {
    pub fn new() -> Row {
        Row { data: vec![Pixel::new()] }
    }
    pub fn with_length(length: usize) -> Row {
        Row { data: vec![Pixel::new(); length] }
    }
    pub fn append(&mut self, pixel: Pixel) {
        self.data.push(pixel)
    }
    pub fn from(data: &[u8]) -> Row {
        let mut row = Row::new();
        for i in data.chunks(4) {
            row.append(Pixel::from(i))
        }
        row
    }
    pub fn get_pixel(&mut self, index: usize) -> &mut Pixel {
        &mut self.data[index - 1]
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

#[allow(dead_code)]
pub struct ImageBuffer {
    data: Vec<Row>
}
impl ImageBuffer {
    pub fn new() -> ImageBuffer {
        ImageBuffer { data: vec![Row::new()] }
    }
    pub fn with_dimensions(width: usize, height: usize) -> ImageBuffer {
        ImageBuffer { data: vec![Row::with_length(width); height] }
    }
    pub fn get_row(&mut self, index: usize) -> &mut Row {
        &mut self.data[index - 1]
    }
    pub fn as_slice(&self) {
        
    }
}
impl fmt::Debug for ImageBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f.write_str("ImageBuffer ");
        f.debug_list().entries(self.data.iter()).finish()
    }
}
