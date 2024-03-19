//! This module defines structs and functions for parsing the Manifest.
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::*;

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

/// Parses the Manifest file
pub fn parse_manifest<T>(filename: T) -> Manifest
where
    T: AsRef<Path>,
{
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut file = String::new();
    let _ = reader.read_to_string(&mut file);
    let manifest: Manifest = toml::from_str(&file).unwrap();
    manifest
}
