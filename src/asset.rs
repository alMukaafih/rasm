//! This module defines assets loadable by the generator.
use std::path::PathBuf;
use std::collections::HashMap;

/// Assets Map;
type Assets = HashMap<String, Box<dyn Asset>>;

/// An Asset.
pub trait Asset {
    /// Retrieves the asset.
    fn get(&mut self) -> &mut Self;
    /// Loads the asset from Path.
    fn load(src: PathBuf) -> Self;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Svg Asset.
pub struct Svg {
    id: String,
    src: PathBuf,
    content: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// Font Asset.
pub struct Font {
    id: String,
    src: PathBuf,
    content: String,
}
