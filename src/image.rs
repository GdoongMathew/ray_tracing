use image;
use Vec3d as Color;

use crate::vec3d::Vec3d;
use crate::ray::Interval;


pub fn write_image(path: &str, pixels: &Vec<Color>, width: i32, height: i32) {
    let mut img = image::ImageBuffer::new(width as u32, height as u32);

    let color_interval = Interval { min: 0.0, max: 1.0 };

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (y * width as u32 + x) as usize;

        let color = Vec3d::new(
            color_interval.clamp(pixels[index].x()),
            color_interval.clamp(pixels[index].y()),
            color_interval.clamp(pixels[index].z()),
        ) * 256.0;

        *pixel = image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]);
    }

    img.save(path).unwrap();
}