use crate::image::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Point {
    x: usize,
    y: usize,
    color: Pixel,
}
impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0, color: Pixel::new() }
    }
}
impl From<(usize, usize)> for Point {
    fn from(point: (usize, usize)) -> Self {
        Point { x: point.0, y: point.1, color: Pixel::new() }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Rectangle {
    x: usize,
    y: usize,
    color: Pixel,
    width: usize,
    height: usize,
}
impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            color: Pixel::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn with_coordinates(
        a: (usize, usize),
        b: (usize, usize),
        _c: (usize, usize),
        d: (usize, usize)
    ) -> Rectangle {
        let pixel: &[u8] = &[0,0,0,255];
        Rectangle {
            x: a.0,
            y: a.1,
            color: Pixel::from(pixel),
            width: b.1 - a.1,
            height: d.0 - a.0,
        }
    }
    pub fn set_color(&mut self, pixel: Pixel) {
        self.color = pixel
    }
    pub fn paste(&self, img: &mut Image) {
        let mut layer = Layer::with_dimensions(img.width(), img.height());
        for rows in 0..self.height {
        for pixels in 0..self.width {
            layer[self.x + rows][self.y + pixels]
                = self.color;
        }
        }
        img.add_layer(layer)
    }
}