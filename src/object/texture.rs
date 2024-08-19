use crate::vec3d::Vec3d;
use std::sync::Arc;


pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Vec3d;
}


pub struct SolidColor {
    color: Vec3d,
}

impl SolidColor {
    pub fn new(color: Vec3d) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3d) -> Vec3d {
        self.color
    }
}


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
        let even = Arc::new(Box::new(SolidColor::new(color1)));
        let odd = Arc::new(Box::new(SolidColor::new(color2)));
        Self::new(even, odd, scale)
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