pub mod file;
pub mod format;
/// Definition of an Image.
pub mod image;
pub mod parse;
pub mod object;
pub mod util;
//pub mod palette;
#[cfg(test)]
pub mod tests;

use crate::file::*;
use crate::parse::*;
use crate::util::*;
use imghdr::*;
use std::borrow::Cow::Borrowed;
use std::env;
use std::path::*;

fn parse_rect(canvas: &mut Canvas, object: Object) {
    canvas.new_rect(
        object.origin.unwrap(),
        object.offset.unwrap(),
        object.color.unwrap(),
    );
}

fn parse_image(canvas: &mut Canvas, object: Object, mut file: PathBuf) {
    let width = canvas.width;
    let height = canvas.height;
    file.pop();
    file.push(object.src.unwrap());
    let path = file.to_str().unwrap();
    let file_data = match from_file(path) {
        Ok(Some(Type::Png)) => from_png(file),
        Ok(Some(Type::Jpeg)) => from_jpg(file),
        Ok(..) => panic!("unknown format"),
        Err(e) => panic!("Some error happened: {:?}", e),
    };

    let image = canvas.add_image(file_data.0, object.origin.unwrap(), file_data.1);
    if object.resize.is_some() {
        let scale = object.resize.unwrap();
        let w2 = (width as f64 * (scale[0] / 100.0)) as usize;
        let h2 = (height as f64 * (scale[1] / 100.0)) as usize;
        image.resize([w2, h2]);
        //println!("{}, {}", w2, h2);
    }
}

fn main() {
    let mut args = env::args();
    if args.len() - 1 < 1 {
        return;
    }
    let _file = args.nth(1).unwrap();

    let mut __file = PathBuf::from(_file);
    let mut file;
    if __file.is_relative() {
        file = env::current_dir().unwrap();
        file.push(__file)
    } else {
        file = __file
    }

    let mut filename = Borrowed("rasm-design");
    let temp = file.clone();
    let tmp;
    // if it's a directory use it as output name.
    if file.is_dir() {
        filename = temp.file_name().unwrap().to_string_lossy();
        file.push("Rasm.toml")
    }
    // if it's regular file use it as output name.
    else if !file.ends_with(".rasm.toml") {
        file.set_extension("rasm.toml");
        if file.is_file() {
            tmp = temp.file_name().unwrap().to_string_lossy();
            filename = Borrowed(tmp.strip_suffix(".rasm.toml").unwrap())
        }
    } else {
        panic!("Err")
    }
    let input = parse_rasm(&file);
    let mut canvas = Canvas::new(input.format, input.size[0], input.size[1]);
    canvas.new_rect((0.0, 0.0), (100.0, 100.0), input.color);
    for object in input.objects {
        match object.name.as_str() {
            "rect" => parse_rect(&mut canvas, object),
            "image" => parse_image(&mut canvas, object, file.clone()),
            &_ => panic!("unknown object"),
        }
    }
    canvas.save(&filename);
}
