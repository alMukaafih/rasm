use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};

use resize::Pixel::RGBA8;
use resize::Type::Lanczos3;
//use rgb::RGBA8;
use rgb::FromSlice;

use crate::image::*;
//use crate::format::*;
use crate::util::*;

pub trait Diagram {
    fn draw(&mut self, canvas: &mut Canvas);
    fn resize(&mut self, scale: [f64; 2]);
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}
impl From<(usize, usize)> for Point {
    fn from(point: (usize, usize)) -> Self {
        Point { x: point.0, y: point.1 }
    }
}
impl Index<usize> for Point {
    type Output = usize;
    
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2_usize.. => panic!("index out of bounds: the len is 2 but the index is {}", idx),
        }
    }
}
impl IndexMut<usize> for Point {
    //type Output = Pixel;
    
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2_usize.. => panic!("index out of bounds: the len is 2 but the index is {}", idx)
        }
    }
    
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Rect {
    origin: Point,
    color: Pixel,
    width: usize,
    height: usize,
}
impl Rect {
    pub fn new() -> Rect {
        Rect {
            origin: Point::new(),
            color: Pixel::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn with_coordinates(a: (usize, usize), c: (usize, usize)) -> Rect {
        let pixel: &[u8] = &[0,0,0,255];
        Rect {
            origin: Point::from(a),
            color: Pixel::from(pixel),
            width: c.1 - a.1,
            height: c.0 - a.0,
        }
    }
    pub fn set_color(&mut self, pixel: Pixel) {
        self.color = pixel
    }
    pub fn paste(&self, img: &mut Image) {
        let mut layer = Layer::with_dimensions(img.width(), img.height());
        for rows in 0..self.height {
        for pixels in 0..self.width {
            layer[self.origin[0] + rows][self.origin[1] + pixels]
                = self.color;
        }
        }
        img.add_layer(layer)
    }
}
impl From<(usize, usize ,usize, usize, u8, u8, u8, u8)> for Rect {
    fn from(i: (usize, usize, usize, usize, u8, u8, u8, u8)) -> Self {
        let pixel: &[u8] = &[i.4,i.5,i.6,i.7];
        Rect {
            origin: Point::from((i.0, i.1)),
            color: Pixel::from(pixel),
            width: i.3 - i.1,
            height: i.2 - i.0,
        }
    }
}
impl Diagram for Rect {
    fn draw(&mut self, canvas: &mut Canvas) {
        let layer = &mut canvas.fmt().image()[0];
        for rows in 0..self.height {
            for pixels in 0..self.width {
                layer[self.origin[0] + rows][self.origin[1] + pixels]
                    += self.color;
            }
        }
    }
    fn resize(&mut self, scale: [f64; 2]) {
        
    }
}

#[allow(dead_code)]
//#[derive(Clone, Debug)]
pub struct Photo {
    buf: VecDeque<u8>,
    width: usize,
    origin: Point,
}
impl From<(Vec<u8>, usize, (usize, usize))> for Photo {
    fn from(img: (Vec<u8>, usize, (usize, usize))) -> Self {
        Photo {
            buf: VecDeque::from(img.0),
            width: img.1,
            origin: Point::from(img.2),
        }
    }
}
impl Diagram for Photo {
    fn draw(&mut self, canvas: &mut Canvas) {
        let layer = &mut canvas.fmt().image()[0];
        let height = (self.buf.len() / (4 * self.width)) as usize;
        for rows in 0..height {
            for pixels in 0..self.width {
                layer[self.origin[0] + rows][self.origin[1] + pixels]
                    += Pixel::from(&[
                        self.buf.pop_front().unwrap(),
                        self.buf.pop_front().unwrap(),
                        self.buf.pop_front().unwrap(),
                        self.buf.pop_front().unwrap()
                    ]);
            }
        }
    }
    fn resize(&mut self, scale: [f64; 2]) {
        let w1 = self.width;
        let h1 = (self.buf.len()/(4*self.width)) as usize;
        let w2 = (w1 as f64 * (scale[0]/w1 as f64)) as usize;
        let h2;
        if scale[1] != 0.0 {
            h2 = (h1 as f64 * (scale[1]/h1 as f64)) as usize;
        } else {
            h2 = (h1 as f64 * (scale[0]/w1 as f64)) as usize;
        }
    
            // Don't forget to fill `src` with image data (RGB8).
        let _src = Vec::from(self.buf.make_contiguous());
        let src = _src.as_rgba();
        // Destination buffer. Must be mutable.
        let mut _dst = vec![0;w2*h2*4];
        let mut dst = _dst.as_rgba_mut();
        // Create reusable instance.
        let mut resizer = resize::new(
            w1, h1,
            w1, h2,
            RGBA8, 
            Lanczos3
            ).unwrap();
        // Do resize without heap allocations.
        // Might be executed multiple times for different `src` or `dst`.
        let r = resizer.resize(&src, &mut dst);
        println!("{:?}", r)
        //self.buf = VecDeque::from(dst);
    }
}