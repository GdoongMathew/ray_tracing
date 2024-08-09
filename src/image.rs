use image::codecs::png::PngEncoder;

use crate::vec3d::Vec3d;
use Vec3d as Color;


pub fn write_image(path: &str, pixels: &Vec<Color>, width: i32, height: i32) {
    let mut img = image::ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (y * width as u32 + x) as usize;
        let color = *&pixels[index] * 255.999;

        *pixel = image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]);
    }

    img.save(path).unwrap();
}