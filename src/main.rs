#![warn(missing_docs)]
//! Program for Generating Images from a Manifest.
//!
//! # Usage
//!
//! ```bash
//! rasm hadith.rasm.toml
//! rasm hadith/
//! ```
//! 


pub mod file;
pub mod format;
pub mod image;
pub mod parse;
pub mod object;
pub mod util;
pub mod asset;
//pub mod palette;
#[cfg(test)]
pub mod tests;

use crate::file::*;
use crate::image::*;
use crate::object::*;
use crate::parse::*;
use crate::util::*;

use file_format::{FileFormat, Kind};

use std::borrow::Cow::Borrowed;
use std::env;
use std::path::PathBuf;

/// The Generator.
///
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
    else if !file.ends_with(".manifest.toml") {
        file.set_extension("manifest.toml");
        if file.is_file() {
            tmp = temp.file_name().unwrap().to_string_lossy();
            filename = Borrowed(tmp.strip_suffix(".manifest.toml").unwrap())
        }
    } else {
        panic!("Err")
    }
    // parse the Manifest file
    let manifest = parse_manifest(&file);
    let mut canvas = Canvas::new(manifest.format, manifest.size[0], manifest.size[1]);
    canvas.new_rect((0.0, 0.0), (100.0, 100.0), manifest.color);
    if manifest.assets.is_some() {
        parse_assets(manifest.assets.unwrap());
    }
    for object_info in manifest.objects {
        match object_info.name.as_str() {
            "rect" => parse_rect(&mut canvas, object_info),
            "image" => parse_image(&mut canvas, object_info, file.clone()),
            &_ => panic!("unknown object"),
        }
    }
    canvas.save(&filename);
}
