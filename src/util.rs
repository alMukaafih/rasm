//! This module defines useful utilities used by the generator.
use std::collections::VecDeque;
use file_format::{FileFormat, Kind};
use std::path::PathBuf;

//use crate::image::*;
use crate::format::*;
use crate::image::*;
use crate::object::*;
use crate::parse::*;

#[allow(dead_code)]
//#[derive(Clone, Debug)]
/// Canvas for drawing Objects.
pub struct Canvas {
    /// Width of Canvas.
    pub width: usize,
    /// Height of Canvas.
    pub height: usize,
    format: Box<dyn Format>,
    shapes: VecDeque<Box<dyn Object>>,
}
impl Canvas {
    /// Creates a new Canvas.
    pub fn new<T>(fmt: T, width: usize, height: usize) -> Canvas
    where
        T: AsRef<str>,
    {
        let img: Box<dyn Format>;
        if fmt.as_ref() == "png" {
            img = Box::new(Png::new(width, height))
        } else if fmt.as_ref() == "jpg" {
            img = Box::new(Jpg::new(width, height))
        } else {
            panic!("unknown image format")
        }
        Canvas {
            width,
            height,
            format: img,
            shapes: VecDeque::new(),
        }
    }
    /// Creates a new [Rectangle][R] in Canvas.
    ///
    /// [R]: Rect
    pub fn new_rect(
        &mut self,
        a: (f64, f64),
        c: (f64, f64),
        color: [u8; 4],
    ) -> &mut Box<dyn Object> {
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
    /// Adds an [Image] to the Canvas.
    pub fn add_image(
        &mut self,
        origin: (f64, f64),
        mut image: Image,
    ) -> &mut Box<dyn Object> {
        let ox = (self.width as f64 * (origin.0 / 100.0)) as usize;
        let oy = (self.height as f64 * (origin.1 / 100.0)) as usize;
        image.origin = Point::from((ox, oy));
        self.shapes.push_back(Box::new(image));
        let idx = self.shapes.len();
        &mut self.shapes[idx - 1]
    }
    /// Retrieve the Output format for Canvas.
    pub fn fmt(&mut self) -> &mut Box<dyn Format> {
        &mut self.format
    }
    /// Saves the Canvas as format file in disk.
    pub fn save(&mut self, filename: &str) {
        for _i in 0..self.shapes.len() {
            let mut obj = self.shapes.pop_front().unwrap();
            obj.draw(self)
        }
        self.format.write(filename)
    }
}

#[allow(unused_variables)]
/// Parses assets in the Manifest. 
pub fn parse_assets(assets_info: Vec<AssetInfo>) {
    for asset_info in assets_info {
        
    }
}

/// Parses a [Rectangle][R].
///
/// [R]: Rect
pub fn parse_rect(canvas: &mut Canvas, object_info: ObjectInfo) {
    canvas.new_rect(
        object_info.origin.unwrap(),
        object_info.offset.unwrap(),
        object_info.color.unwrap(),
    );
}

/// Parses an [Image].
pub fn parse_image(canvas: &mut Canvas, object_info: ObjectInfo, mut file: PathBuf) {
    let width = canvas.width;
    let height = canvas.height;
    file.pop();
    file.push(object_info.src.unwrap());
    //let path = file.to_str().unwrap();
    let fmt = FileFormat::from_file(&file).unwrap();
    let img = match fmt.kind() {
        Kind::Image => Image::from_file(file),
        _ => panic!("{} is not an Image file", file.to_str().unwrap()),
    };

    let image = canvas.add_image(object_info.origin.unwrap(), img);
    if object_info.resize.is_some() {
        let scale = object_info.resize.unwrap();
        let w2 = (width as f64 * (scale[0] / 100.0)) as usize;
        let h2 = (height as f64 * (scale[1] / 100.0)) as usize;
        image.resize([w2, h2]);
        //println!("{}, {}", w2, h2);
    }
}