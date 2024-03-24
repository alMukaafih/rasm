//! This module defines assets loadable by the generator.
use std::collections::HashMap;
use std::path::PathBuf;

use svg::node::element::tag::Path;
use svg::parser::Event;
#[allow(unused_imports)]
use swash::zeno::{Mask, PathData};

//use crate::object::*;
use crate::image::*;

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
impl Svg {
    /// Loads Svg from file.
    pub fn load(src: PathBuf) -> Self {
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
    /// Renders Svg as bytes.
    pub fn render(&self, _scale: [usize; 2], _text: Option<String>) -> Vec<u8> {
        vec![]
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Font Asset.
pub struct Font {
    id: String,
    src: PathBuf,
}

#[allow(dead_code)]
/// Assets Map;
pub struct Assets {
    assets: Vec<String>,
    fonts: HashMap<String, Font>,
    images: HashMap<String, Image>,
    svgs: HashMap<String, Svg>,
}
impl Assets {
    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.assets.len()
    }
    /// Creates an empty Assets Map.
    ///
    /// The assets map is initially created with a capacity of 0, so it will not allocate until it is first inserted into.
    pub fn new() -> Assets {
        Assets {
            assets: Vec::new(),
            fonts: HashMap::new(),
            images: HashMap::new(),
            svgs: HashMap::new(),
        }
    }
    /// Creates an empty HashMap with at least the specified capacity.
    ///
    /// The hash map will be able to hold at least capacity elements without reallocating. This method is allowed to allocate for more elements than capacity. If capacity is 0, the hash map will not allocate.
    pub fn with_capacity(capacity: usize) -> Assets {
        Assets {
            assets: Vec::with_capacity(capacity),
            fonts: HashMap::with_capacity(capacity),
            images: HashMap::with_capacity(capacity),
            svgs: HashMap::with_capacity(capacity),
        }
    }
}

/// Asset Map Methods
pub trait AssetsMethods<T> {
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    fn insert(&mut self, k: &str, v: T) -> Option<T>;
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, k: &str) -> Option<&T>;
    /// Returns a mutable reference to the value corresponding to the key.
    fn get_mut(&mut self, k: &str) -> Option<&mut T>;
    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    fn remove(&mut self, k: &str) -> Option<T>;
}

impl AssetsMethods<Font> for Assets {
    fn insert(&mut self, k: &str, v: Font) -> Option<Font> {
        self.assets.push(k.to_string());
        self.fonts.insert(k.to_string(), v)
    }
    fn get(&self, k: &str) -> Option<&Font> {
        self.fonts.get(k)
    }
    fn get_mut(&mut self, k: &str) -> Option<&mut Font> {
        self.fonts.get_mut(k)
    }
    fn remove(&mut self, k: &str) -> Option<Font> {
        self.fonts.remove(k)
    }
}

impl AssetsMethods<Image> for Assets {
    fn insert(&mut self, k: &str, v: Image) -> Option<Image> {
        self.assets.push(k.to_string());
        self.images.insert(k.to_string(), v)
    }
    fn get(&self, k: &str) -> Option<&Image> {
        self.images.get(k)
    }
    fn get_mut(&mut self, k: &str) -> Option<&mut Image> {
        self.images.get_mut(k)
    }
    fn remove(&mut self, k: &str) -> Option<Image> {
        self.images.remove(k)
    }
}

impl AssetsMethods<Svg> for Assets {
    fn insert(&mut self, k: &str, v: Svg) -> Option<Svg> {
        self.assets.push(k.to_string());
        self.svgs.insert(k.to_string(), v)
    }
    fn get(&self, k: &str) -> Option<&Svg> {
        self.svgs.get(k)
    }
    fn get_mut(&mut self, k: &str) -> Option<&mut Svg> {
        self.svgs.get_mut(k)
    }
    fn remove(&mut self, k: &str) -> Option<Svg> {
        self.svgs.remove(k)
    }
}
