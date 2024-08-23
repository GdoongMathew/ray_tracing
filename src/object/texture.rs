use crate::vec3d::Vec3d;
use std::sync::Arc;
use image;

use std::fmt::{Debug, Formatter};
use image::Pixel;
use crate::ray::Interval;


pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Vec3d;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color: Vec3d,
}

impl SolidColor {
    pub fn new(color: Vec3d) -> Self {
        Self { color }
    }
}

impl Debug for SolidColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SolidColor w color {:?}", self.color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3d) -> Vec3d {
        self.color
    }
}


#[derive(Clone)]
pub struct Checker {
    inv_scale: f64,
    even: Arc<Box<dyn Texture>>,
    odd: Arc<Box<dyn Texture>>,
}

impl Checker {
    pub fn new(even: Arc<Box<dyn Texture>>, odd: Arc<Box<dyn Texture>>, scale: f64) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_color(color1: Vec3d, color2: Vec3d, scale: f64) -> Self {
        let even: Arc<Box<dyn Texture>> = Arc::new(Box::new(SolidColor::new(color1)));
        let odd: Arc<Box<dyn Texture>> = Arc::new(Box::new(SolidColor::new(color2)));
        Self::new(
            even,
            odd,
            scale,
        )
    }
}


impl Debug for Checker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checker w scale {} even {:?} odd {:?}", self.inv_scale, self.even, self.odd)
    }
}


impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Vec3d {
        let p_val = *p * self.inv_scale;

        let x_int = p_val.x().floor() as i32;
        let y_int = p_val.y().floor() as i32;
        let z_int = p_val.z().floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}


struct ImageTexture {
    file: String,
    image: image::RgbImage,
}


impl ImageTexture {
    pub fn new(file: &String) -> Self {
        let image = image::ImageReader::new(file).decode();
        match image {
            Ok(image) => Self { image: image.to_rgb8(), file: file.clone() },
            Err(e) => panic!("Error reading image file {}: {}", file, e),
        }
    }
}


impl Debug for ImageTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image w file {}", self.file)
    }
}


impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Vec3d {
        if self.image.height() <= 0 || self.image.width() <= 0 {
            return Vec3d::new(0.0, 1.0, 1.0);
        }

        let interval = Interval { min: 0.0, max: 1.0 };
        let u = interval.clamp(u);
        let v = 1.0 - interval.clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.get_pixel(i as u32, j as u32).to_rgb();

        Vec3d::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}