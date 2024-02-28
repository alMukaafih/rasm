/// Definition of an Image.
pub mod image;
pub mod shape;
#[cfg(test)]
pub mod tests;

use crate::image::*;

fn main() {
    let mut img = Image::with_dimensions(10, 10);
    let layer = img.get_layer(1);
    let row = layer.get_row(1);
    let pixel = row[0]
        .set_red(100)
        .set_green(50)
        .set_alpha(255);
    pixel[1] = 70;
    
    println!("{:?}", pixel);
   // println!("{:?}", img)
    //println!("{:?}", row)
}
