use image;
use Vec3d as Color;

use crate::vec3d::Vec3d;
use crate::ray::Interval;


fn linear_to_gamma(value: f64) -> f64 {
    if value > 0.0 {value.sqrt()} else {0.0}
}


pub fn write_image(path: &str, pixels: &Vec<Color>, width: i32, height: i32) {
    let mut img = image::ImageBuffer::new(width as u32, height as u32);

    let color_interval = Interval { min: 0.0, max: 0.999 };

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (y * width as u32 + x) as usize;

        let r = linear_to_gamma(pixels[index].x());
        let g = linear_to_gamma(pixels[index].y());
        let b = linear_to_gamma(pixels[index].z());

        let color = Vec3d::new(
            color_interval.clamp(r),
            color_interval.clamp(g),
            color_interval.clamp(b),
        ) * 256.0;

        *pixel = image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8]);
    }

    img.save(path).unwrap();
}