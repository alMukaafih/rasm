//! This module defines structs and functions for parsing the Manifest.
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::*;
//use std::borrow::Cow::Borrowed;
use std::env::Args;
use std::env;

use crate::util::*;

#[allow(unused_imports)]
use crate::object::*;

#[derive(Debug, Deserialize)]
/// The representation of the Manifest file.
pub struct Manifest {
    /// The format of the generated Image.
    pub format: String,
    /// Size of the generated Image.
    pub size: [usize; 2],
    /// Background Color of the generated Image.
    pub color: [u8; 4],
    /// Assets used during generation of the Image.
    pub assets: Option<Vec<AssetInfo>>,
    /// Objects in the generated Image.
    pub objects: Vec<ObjectInfo>,
}

#[derive(Debug, Deserialize)]
/// This is the representation of an Asset's details.
pub struct AssetInfo {
    /// String for hashing the Asset.
    pub id: String,
    /// Source path of the Asset
    pub src: PathBuf,
}

#[derive(Debug, Deserialize)]
/// This is the representation of an Object's details.
pub struct ObjectInfo {
    /// Name of [Object].
    pub name: String,
    /// Source path of [Object].
    pub src: Option<PathBuf>,
    /// Asset id of [Object]
    pub asset: Option<String>,
    /// Color of [Object].
    pub color: Option<[u8; 4]>,
    /// Text Content of [Object].
    pub content: Option<String>,
    /// Resize [Object] to coordinates with x and y values represented as percentages of width and height of Image respectively.
    /// If either of x value, y value is zero, Ratio of [Object] is maintained. 
    pub resize: Option<[f64; 2]>,
    /// Origin of [Object] with x and y values represented as percentages of width and height of Image respectively.
    pub origin: Option<(f64, f64)>,
    /// Coordinates of end point of diagonal from origin.
    pub offset: Option<(f64, f64)>,
}

/// Args Parser.qa
pub fn parse_args(mut args: Args) -> PathBuf {
    let file_from_arg;
    if args.len() - 1 < 1 {
        std::process::exit(1);
    }
    else {
        file_from_arg = args.nth(1).unwrap();
    }

    let file_path = PathBuf::from(file_from_arg);
    let mut file;
    if file_path.is_relative() {
        file = env::current_dir().unwrap();
        file.push(file_path)
    } else {
        file = file_path
    }

    //let mut filename = Borrowed("rasm-design");
    //let temp = file.clone();
    //let tmp;
    // if it's a directory use it as output name.
    if file.is_dir() {
        //filename = temp.file_name().unwrap().to_string_lossy();
        file.push("Rasm.toml")
    }
    // if it's regular file use it as output name.
    else if !file.ends_with(".manifest.toml") {
        file.set_extension("manifest.toml");
        if file.is_file() {
            //tmp = temp.file_name().unwrap().to_string_lossy();
            //filename = Borrowed(tmp.strip_suffix(".manifest.toml").unwrap())
        }
    } else {
        panic!("Err")
    }
    file
}

/// Parses the Manifest file
pub fn parse_manifest(file: PathBuf) {
    let f = File::open(&file).unwrap();
    let mut reader = BufReader::new(f);

    let mut read_file = String::new();
    let _ = reader.read_to_string(&mut read_file);
    let parsed_file = toml::from_str::<Manifest>(&read_file);
    let manifest;
    match parsed_file {
        Ok(_manifest) => {
            manifest = _manifest;
        },
        Err(err) => {
            println!("{}", err.message());
            println!("{:?}", err.span());
            std::process::exit(1);
        }
    }
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
    canvas.save(file.to_str().unwrap());
}
