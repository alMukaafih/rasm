use crate::image::*;
use crate::object::*;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[test]
fn create_png() {
    let mut img = Image::new(1080, 1080);
    let mut rect1 = Rect::with_coordinates((0, 0), (1080, 270));
    let mut rect2 = Rect::with_coordinates((0, 270), (1080, 540));
    let mut rect3 = Rect::with_coordinates((0, 540), (1080, 810));
    let mut rect4 = Rect::with_coordinates((0, 810), (1080, 1080));
    rect1.set_color(Pixel::from(&[0, 75, 150, 255]));
    rect2.set_color(Pixel::from(&[150, 0, 150, 255]));
    rect3.set_color(Pixel::from(&[150, 75, 0, 255]));
    rect4.set_color(Pixel::from(&[0, 150, 0, 255]));
    rect1.paste(&mut img);
    rect2.paste(&mut img);
    rect3.paste(&mut img);
    rect4.paste(&mut img);

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
