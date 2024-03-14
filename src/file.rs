use std::fs::read;
use std::path::*;

use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;

pub fn from_png<R: AsRef<Path>>(filename: R) -> ((usize, usize), Vec<u8>) {
    let file_contents = read(filename).unwrap();
    let options = DecoderOptions::default()
        .png_set_add_alpha_channel(true)
        .png_set_strip_to_8bit(true);
    // use the above option to decode
    let mut decoder = PngDecoder::new_with_options(&file_contents, options);

    let buf = decoder.decode_raw().unwrap();
    let info = decoder.get_info().unwrap();
    let width = info.width;
    let height = info.height;
    ((width, height), buf)
}

pub fn from_jpg<R: AsRef<Path>>(filename: R) -> ((usize, usize), Vec<u8>) {
    let file_contents = read(filename).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);

    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    let buf = decoder.decode().unwrap();
    let info = decoder.info().unwrap();
    let width = info.width;
    let height = info.height;
    ((width.into(), height.into()), buf)
}
