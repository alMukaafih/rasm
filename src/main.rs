#![warn(missing_docs)]
//! Program for Generating Images from a Manifest.
//!
//! # Usage
//!
//! ```bash
//! rasm hadith.rasm.toml
//!
//! rasm hadith/
//! ```
//!

pub mod asset;
pub mod format;
pub mod image;
pub mod object;
pub mod parse;
pub mod util;
//pub mod palette;
#[cfg(test)]
pub mod tests;

use std::env;
use crate::parse::*;

/// The Generator.
///
fn main() {
    let file = parse_args(env::args());
    // parse the Manifest file
    parse_manifest(file);
}
