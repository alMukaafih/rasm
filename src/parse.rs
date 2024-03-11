use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::*;

#[derive(Debug, Deserialize)]
pub struct Rasm {
    pub format: String,
    pub size: [usize; 2],
    pub color: [u8; 4],
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    pub name: String,
    pub path: Option<String>,
    pub color: Option<[u8; 4]>,
    pub content: Option<String>,
    pub resize: Option<[f64; 2]>,
    pub origin: Option<(f64, f64)>,
    pub offset: Option<(f64, f64)>,
}

pub fn parse_rasm<T>(filename: T) -> Rasm
where
    T: AsRef<Path>,
{
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut file = String::new();
    let _ = reader.read_to_string(&mut file);
    let input: Rasm = toml::from_str(file.as_ref()).unwrap();
    input
}
