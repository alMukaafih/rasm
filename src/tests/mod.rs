use crate::image::*;
use crate::shape::*;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

#[test]
fn create_png() {
    let mut img = Image::with_dimensions(1080, 1080);
    let mut rect = Rectangle::with_coordinates(
        (40,40), (0,1040), (1040,1040), (1040,40)
    );
    rect.set_color(Pixel::from(&[255,0,0,255]));
    rect.paste(&mut img);

    let path = Path::new(r"test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 1080, 1080); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    //encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    //encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
//     let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
//         (0.31270, 0.32900),
//         (0.64000, 0.33000),
//         (0.30000, 0.60000),
//         (0.15000, 0.06000)
//     );
    //encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    
    //let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    //writer.write_image_data(&data).unwrap(); // Save
    writer.write_image_data(img.to_vec().as_slice()).unwrap(); // Save
    //println!("{:?}", rect[0]);
    //println!("{:?}", img[0][0][0][0]);
    //println!("{:?}", img.to_vec().as_slice()[0])
}