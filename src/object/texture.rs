use crate::vec3d::Vec3d;
use std::sync::Arc;

use std::fmt::{Debug, Formatter};


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
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>, scale: f64) -> Self {
        let even = Arc::new(even);
        let odd = Arc::new(odd);
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_color(color1: Vec3d, color2: Vec3d, scale: f64) -> Self {
        Self::new(
            Box::new(SolidColor::new(color1)),
            Box::new(SolidColor::new(color2)),
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