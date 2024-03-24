//! This module defines assets loadable by the generator.
use std::collections::HashMap;
use std::path::PathBuf;

use svg::node::element::tag::Path;
use svg::parser::Event;
#[allow(unused_imports)]
use swash::zeno::{Mask, PathData};

//use crate::object::*;
//use crate::util::*;

/// An Asset.
pub trait Asset {
    /// Loads the asset from Path.
    fn load(src: PathBuf) -> Self where Self: Sized;
    /// Renders using scale.
    fn render(&self, scale: [usize; 2], text: Option<String>) -> Vec<u8>;
    /// It is a Font asset?
    fn is_font(&self) -> bool {
        false
    }
    /// It is a Raster asset?
    fn is_raster(&self) -> bool {
        false
    }
    /// It is a Vector asset?
    fn is_vector(&self) -> bool {
        false
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
                    data.push(' ');
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

#[allow(dead_code)]
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
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, k: &str) -> Option<&Box<dyn Asset>> {
        let v = self.assets.get(k);
        match v {
            Some(Item::Font) => return self.fonts.get(k),
            Some(Item::Raster) => return self.rasters.get(k),
            Some(Item::Vector) => return self.vectors.get(k),
            None => None,
        }
    }
    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, k: &str) -> Option<&mut Box<dyn Asset>> {
        let v = self.assets.get_mut(k);
        match v {
            Some(Item::Font) => return self.fonts.get_mut(k),
            Some(Item::Raster) => return self.rasters.get_mut(k),
            Some(Item::Vector) => return self.vectors.get_mut(k),
            None => None,
        }
    }
    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.assets.len()
    }
    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, k: &str, v: Box<dyn Asset>) -> Option<Box<dyn Asset>> {
        if v.is_font() {
            self.assets.insert(k.to_string(), Item::Font);
            self.fonts.insert(k.to_string(), v)
        } else if v.is_raster() {
            self.assets.insert(k.to_string(), Item::Raster);
            self.rasters.insert(k.to_string(), v)
        } else if v.is_vector() {
            self.assets.insert(k.to_string(), Item::Vector);
            self.vectors.insert(k.to_string(), v)
        } else {
            None
        }
    }
    /// Creates an empty Assets.
    ///
    /// The assets map is initially created with a capacity of 0, so it will not allocate until it is first inserted into.
    pub fn new() -> Assets {
        Assets {
            assets: HashMap::new(),
            fonts: HashMap::new(),
            rasters: HashMap::new(),
            vectors: HashMap::new(),
        }
    }
    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove(&mut self, k: &str) -> Option<Box<dyn Asset>> {
        let v = self.assets.remove(k)
    }
}
