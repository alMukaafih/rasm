//! This module defines objects for the generator.
use std::ops::{Add, AddAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Sub, SubAssign};

use resize::Pixel::RGBA8;
use resize::Type::Lanczos3;
//use rgb::RGBA8;
use rgb::FromSlice;
use swash::scale::StrikeWith;
use swash::scale::{image::Image as Img, Render, ScaleContext, Source};
use swash::{zeno, FontRef, GlyphId};
use zeno::{Format, Vector};

use crate::image::*;
//use crate::format::*;
use crate::util::*;

/// The definition of an Object.
pub trait Object {
    /// Draws the Object on the [Canvas].
    fn draw(&mut self, canvas: &mut Canvas);
    /// Resizes the [Object] using scale.
    fn resize(&mut self, scale: [usize; 2]);
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
/// A Point on a Cartesian plane.
pub struct Point {
    /// x coordinate.
    pub x: usize,
    /// y coordinate.
    pub y: usize,
}
impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Point {
    /// Creates a new Point.
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}
impl From<(usize, usize)> for Point {
    fn from(point: (usize, usize)) -> Self {
        Point {
            x: point.0,
            y: point.1,
        }
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
            2_usize.. => panic!("index out of bounds: the len is 2 but the index is {}", idx),
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
/// A Rectangle on a Cartesian Plane.
pub struct Rect {
    /// Origin of Rectangle.
    pub origin: Point,
    /// Color of Rectangle.
    pub color: Pixel,
    /// Width of Rectangle.
    pub width: usize,
    /// Height of Rectangle.
    pub height: usize,
}
impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}

impl Rect {
    /// Creates a new Rectangle.
    pub fn new() -> Rect {
        Rect {
            origin: Point::new(),
            color: Pixel::new(),
            width: 0,
            height: 0,
        }
    }
    /// Creates Rectangle using coordinates.
    pub fn with_coordinates(a: (usize, usize), c: (usize, usize)) -> Rect {
        let pixel: &[u8] = &[0, 0, 0, 255];
        Rect {
            origin: Point::from(a),
            color: Pixel::from(pixel),
            width: c.1 - a.1,
            height: c.0 - a.0,
        }
    }
    /// Sets color of Rectangle.
    pub fn set_color(&mut self, pixel: Pixel) {
        self.color = pixel
    }
    /// Paste Rectangle on [Image]
    pub fn paste(&self, img: &mut Image) {
        let mut layer = Layer::with_dimensions(img.width(), img.height());
        for rows in 0..self.height {
            for pixels in 0..self.width {
                layer[self.origin[0] + rows][self.origin[1] + pixels] = self.color;
            }
        }
        img.add_layer(layer)
    }
}
impl From<(usize, usize, usize, usize, u8, u8, u8, u8)> for Rect {
    fn from(i: (usize, usize, usize, usize, u8, u8, u8, u8)) -> Self {
        let pixel: &[u8] = &[i.4, i.5, i.6, i.7];
        Rect {
            origin: Point::from((i.0, i.1)),
            color: Pixel::from(pixel),
            width: i.3 - i.1,
            height: i.2 - i.0,
        }
    }
}
impl Object for Rect {
    fn draw(&mut self, canvas: &mut Canvas) {
        let layer = &mut canvas.fmt().image()[0];
        for rows in 0..self.height {
            for pixels in 0..self.width {
                layer[self.origin[0] + rows][self.origin[1] + pixels] += self.color;
            }
        }
    }
    fn resize(&mut self, _scale: [usize; 2]) {}
}
impl Object for Image {
    fn draw(&mut self, canvas: &mut Canvas) {
        let layer = &mut canvas.fmt().image()[0];
        let lay = &self.layers[0];
        for rows in 0..self.height {
            for pixels in 0..self.width {
                layer[self.origin[0] + rows][self.origin[1] + pixels] += lay[rows][pixels];
            }
        }
    }
    fn resize(&mut self, scale: [usize; 2]) {
        let w1 = self.width;
        let h1 = self.height;
        if scale[0] == 0 && scale[1] == 0 {
            panic!()
        }
        let mut w2 = scale[0];
        let mut h2 = scale[1];
        if w1 == w2 && h1 == h2 {
            return;
        }
        if scale[0] == 0 {
            w2 = (w1 as f64 * (h2 as f64 / h1 as f64)) as usize;
        }
        if scale[1] == 0 {
            h2 = (h1 as f64 * (w2 as f64 / w1 as f64)) as usize;
        }
        self.width = w2;
        self.height = h2;

        // Don't forget to fill `src` with image data (RGB8).
        let _src = self.to_vec();
        let src = _src.as_rgba();
        // Destination buffer. Must be mutable.
        let mut _dst = vec![0; w2 * h2 * 4];
        let dst = _dst.as_rgba_mut();
        // Create reusable instance.
        let mut resizer = resize::new(w1, h1, w2, h2, RGBA8, Lanczos3).unwrap();
        // Do resize without heap allocations.
        // Might be executed multiple times for different `src` or `dst`.
        let _ = resizer.resize(src, dst);
        //println!("{:?}", r);
        //println!("{}, {} / {}, {}", w1, h1, w2, h2);
        let mut buf = Vec::new();
        for rgba in dst {
            for i in rgba.iter() {
                buf.push(i)
            }
        }
        self.layers = vec![Layer::from((w2, h2, buf))]
    }
}

#[allow(dead_code)]
#[derive(Clone)]
/// A Text Box.
pub struct TextBox {
    /// Text content.
    pub content: String,
    /// Point of origin.
    pub origin: (usize, usize),
    /// 
    pub offset: (usize, usize),
    /// Image representation of Text box characters
    pub glyphs: Vec<Img>,
}
impl TextBox {
    /// Creates a new Text box.
    pub fn new(content: String, origin: (usize, usize), offset: (usize, usize)) -> TextBox {
        TextBox {
            content,
            origin,
            offset,
            glyphs: vec![],
        }
    }
    /// Render a Glyph character.
    pub fn render_glyph(
        context: &mut ScaleContext,
        font: &FontRef,
        size: f32,
        hint: bool,
        glyph_id: GlyphId,
        x: f32,
        y: f32,
    ) -> Option<Img> {
        // Build the scaler
        let mut scaler = context.builder(*font).size(size).hint(hint).build();
        // Compute the fractional offset-- you'll likely want to quantize this
        // in a real renderer
        let offset = Vector::new(x.fract(), y.fract());
        // Select our source order
        Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ])
        // Select a subpixel format
        .format(Format::Subpixel)
        // Apply the fractional offset
        .offset(offset)
        // Render the image
        .render(&mut scaler, glyph_id)
    }
}
