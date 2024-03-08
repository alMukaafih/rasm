use std::fs::read;

use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;

pub fn from_png<R: AsRef<str>>(filename: R) -> (usize, Vec<u8>) {
    let file_contents = read(filename.as_ref()).unwrap();
    let options = DecoderOptions::default().png_set_add_alpha_channel(true).png_set_strip_to_8bit(true);
    // use the above option to decode
    let mut decoder = PngDecoder::new_with_options(&file_contents, options);

    let buf = decoder.decode_raw().unwrap();
    let width = decoder.get_info().unwrap().width;
    (width as usize, buf) 
}

pub fn from_jpg<R: AsRef<str>>(filename: R) -> (usize, Vec<u8>) {
    let file_contents = read(filename.as_ref()).unwrap();
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA);

    let mut decoder = JpegDecoder::new_with_options(&file_contents, options);
    let buf = decoder.decode().unwrap();
    let width = decoder.info().unwrap().width;
    (width as usize, buf) 
}
