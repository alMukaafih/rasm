pub mod image;
use crate::image::ImageBuffer;

fn main() {
    let mut img = ImageBuffer::with_dimensions(10, 10);
    let row_1 = img.get_row(1);
    let pixel = row_1
        .get_pixel(1)
        .set_red(1)
        .set_alpha(255);
    println!("{:?}", pixel)
}