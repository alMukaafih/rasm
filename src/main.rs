/// Definition of an Image.
pub mod image;
pub mod shape;
pub mod format;
pub mod util;
pub mod parse;
pub mod file;
//pub mod palette;
#[cfg(test)]
pub mod tests;

use crate::parse::*;
use crate::util::*;
use crate::file::*;
use std::env;

fn main() {
    let mut args = env::args();
    if args.len() - 1 < 1 {
        return
    }
    let filename = args.nth(1).unwrap();
    let input = parse_rasm(format!("{filename}.rasm.toml"));
    let mut canvas = Canvas::new("png", input.size[0], input.size[1]);
    let width = canvas.width;
    let height = canvas.height;
    canvas.new_rect(
        (0.0, 0.0),
        (100.0, 100.0),
        input.color
    );
    for shape in input.shapes {
        if shape.name == "rect".to_string() {
            canvas.new_rect(
                shape.origin.unwrap(),
                shape.offset.unwrap(),
                shape.color.unwrap()
            );
            //println!("{:?}", rect);
        }
        if shape.name == "photo".to_string() {
            let file;
            let path = shape.path.unwrap();
            if path.ends_with(".png") {
                file = from_png(path);
            } else if path.ends_with(".jpg") {
                file = from_jpg(path);
            } else {
                panic!("unknown format")
            }
            
            
            let photo = canvas.add_photo(
                file.0,
                shape.origin.unwrap(),
                file.1,
            );
            if shape.resize != None {
                let scale = shape.resize.unwrap();
                let w2 = (width as f64 * (scale[0]/100.0)) as usize;
                let h2 = (height as f64 * (scale[1]/100.0)) as usize;
                photo.resize([w2, h2]);
                println!("{}, {}", w2, h2);
            }
        }
    }
    canvas.save(filename.as_ref());
}
