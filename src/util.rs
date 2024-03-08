use std::collections::VecDeque;

//use crate::image::*;
use crate::format::*;
use crate::shape::*;
use crate::image::*;

#[allow(dead_code)]
//#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    format: Box<dyn Format>,
    shapes: VecDeque<Box<dyn Diagram>>
}
impl Canvas {
    pub fn new<T>(fmt: T, width: usize, height: usize) -> Canvas 
        where T: AsRef<str>
    {
        let img: Box<dyn Format>;
        if fmt.as_ref() == "png" {
            img = Box::new(Png::new(width, height))
        } else {
            panic!("unknown image format")
        }
        Canvas {
            width: width,
            height: height,
            format: img,
            shapes: VecDeque::new(),
        }
    }
    pub fn new_rect(&mut self, a: (f64, f64), c: (f64, f64), color: [u8; 4]) -> &mut Box<dyn Diagram> {
        let ax = (self.width as f64 * (a.0 / 100.0)) as usize;
        let ay = (self.height as f64 * (a.1 / 100.0)) as usize;
        
        let cx = (self.width as f64 * (c.0 / 100.0)) as usize;
        let cy = (self.height as f64 * (c.1 / 100.0)) as usize;
        
        let mut rect = Rect::with_coordinates((ax, ay), (cx, cy));
        rect.set_color(Pixel::from(&color));
        //rect.paste(self.fmt().image());
        self.shapes.push_back(Box::new(rect));
        let idx = self.shapes.len();
        &mut self.shapes[idx - 1]
    }
    pub fn add_photo(&mut self, img: Vec<u8>, width: usize, origin: (f64, f64)) -> &mut Box<dyn Diagram> {
        let height = img.len()/(4*width);
        let ox = (width as f64 * (origin.0 / 100.0)) as usize;
        let oy = (height as f64 * (origin.1 / 100.0)) as usize;
        
        let photo = Photo::from((img, width, (ox, oy)));
        self.shapes.push_back(Box::new(photo));
        let idx = self.shapes.len();
        &mut self.shapes[idx - 1]
    }
    pub fn fmt(&mut self) -> &mut Box<dyn Format> {
        &mut self.format
    }
    pub fn save(&mut self, filename: &str) {
        for _i in 0..self.shapes.len() {
            let mut obj = self.shapes.pop_front().unwrap();
            obj.draw(self)
        }
        self.format.write(filename)
    }
}