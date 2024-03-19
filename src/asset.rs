//! This module defines assets loadable by the generator.
use std::collections::HashMap;
use std::path::PathBuf;

use svg::node::element::tag::Path;
use svg::parser::Event;
#[allow(unused_imports)]
use swash::zeno::{Mask, PathData};

use crate::object::*;
use crate::util::*;

/// An Asset.
pub trait Asset {
    /// Loads the asset from Path.
    fn load(src: PathBuf) -> Self where Self: Sized;
    /// Renders using scale.
    fn render(&self, scale: [usize; 2], text: Option<String>) -> Vec<u8>;
}

pub trait Vector: Asset {
    fn draw(&self, canvas: &mut Canvas);
}

impl<T: Asset> Object for T {
    fn draw(&mut self, canvas: &mut Canvas) {
        
    }
    fn resize(&mut self, scale: [usize; 2]) {
        let asset = self;
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Svg Asset.
pub struct Svg {
    id: String,
    src: PathBuf,
    data: String,
}
#[allow(non_upper_case_globals)]
impl Asset for Svg {
    fn load(src: PathBuf) -> Self {
        let mut content = String::new();
        let mut data = String::new();
        for event in svg::open(&src, &mut content).unwrap() {
            match event {
                Event::Tag(Path, _, attributes) => {
                    data.push_str(attributes.get("d").unwrap());
                    data.push_str(" ");
                },
                _ => {}
            }
        }
        
        let id = "svg".to_string();
        Svg {
            id, src, data
        }
    }
    fn render(&self, scale: [usize; 2], text: Option<String>) -> Vec<u8> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Font Asset.
pub struct Font {
    id: String,
    src: PathBuf,
    content: String,
}

enum Item {
    Font,
    Raster,
    Vector,
}

#[allow(dead_code)]
/// Assets Map;
pub struct Assets {
    assets: HashMap<String, Item>,
    fonts: HashMap<String, Box<dyn Asset>>,
    rasters: HashMap<String, Box<dyn Asset>>,
    vectors: HashMap<String, Box<dyn Asset>>,
}
impl Assets {
    pub fn get(&self, k: &str) -> Option<&Box<dyn Asset>> {
        let v = self.assets.get(k);
        match v {
            Some(Item::Font) => return self.fonts.get(k),
            Some(Item::Raster) => return self.rasters.get(k),
            Some(Item::Vector) => return self.vectors.get(k),
            None => return None,
        }
    }
}
