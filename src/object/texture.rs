use crate::vec3d::{Vec3d, Color, dot};
use std::sync::Arc;
use image;

use std::fmt::{Debug, Formatter};
use image::{GenericImageView, Pixel};
use crate::ray::Interval;

use rand::Rng;


pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Color;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color: Color,
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
    fn value(&self, _u: f64, _v: f64, _p: &Vec3d) -> Color {
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
    fn value(&self, u: f64, v: f64, p: &Vec3d) -> Color {
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


pub struct ImageTexture {
    file: String,
    image: image::DynamicImage,
}


impl ImageTexture {
    pub fn new(file: &String) -> Self {
        let image = image::open(file);
        match image {
            Ok(image) => Self { file: file.clone(), image },
            Err(e) => panic!("Could not open image file {}: {}", file, e),
        }
    }
}


impl Debug for ImageTexture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image w file {}", self.file)
    }
}


impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3d) -> Color {
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


#[derive(Debug)]
pub struct PerlinTexture {
    point_count: usize,
    rand_vec3d: Vec<Vec3d>,

    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,

    scale: f64,
}


impl PerlinTexture {
    pub fn new(scale: f64) -> Self {
        let mut rng = rand::thread_rng();

        let point_count = 256;
        // let rand_float: Vec<f64> = (0..point_count).map(|_| rng.gen_range(0.0..1.0)).collect();
        let rand_vec3d: Vec<Vec3d> = (0..point_count).map(|_| Vec3d::gen_range(-1.0, 1.0).unit_vector()).collect();

        let perm_x: Vec<i32> = (0..point_count).collect();
        let perm_y: Vec<i32> = perm_x.clone();
        let perm_z: Vec<i32> = perm_x.clone();

        let count = point_count as usize;

        Self {
            point_count: count,
            rand_vec3d,
            perm_x: Self::permute(perm_x, point_count),
            perm_y: Self::permute(perm_y, point_count),
            perm_z: Self::permute(perm_z, point_count),
            scale,
        }
    }

    pub fn noise(&self, point: &Vec3d) -> f64 {
        let new_p = point.map(|x| x - x.floor());

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c: Vec<Vec<Vec<Vec3d>>> = vec![vec![vec![Vec3d::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] =
                        self.rand_vec3d[(self.perm_x[((i + di) & 255) as usize] ^ self.perm_y[((j + dj) & 255) as usize] ^ self.perm_z[((k + dk) & 255) as usize]) as usize];
                }
            }
        }

        Self::perlin_interpolate(c, new_p)
    }

    fn perlin_interpolate(c: Vec<Vec<Vec<Vec3d>>>, u: Vec3d) -> f64 {

        let new_u = u * u * (3.0 - 2.0 * u);

        let mut accum = 0.0;
        let inv_u = -new_u + 1.0;
        let ones = Vec3d::new(1.0, 1.0, 1.0);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = u - Vec3d::new(i as f64, j as f64, k as f64);
                    let coord = Vec3d::new(i as f64, j as f64, k as f64);
                    let vec = coord * u + (ones - coord) * inv_u;

                    accum += dot(&c[i][j][k], &weight_v) * vec.x() * vec.y() * vec.z();
                }
            }
        }
        accum
    }

    fn permute(mut p: Vec<i32>, n: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let target = rng.gen_range(0..i) as usize;
            let i = i as usize;

            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
        p
    }

    fn turbulence(&self, point: &Vec3d, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}


impl Texture for PerlinTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3d) -> Color {
        Vec3d::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z() + 10.0 * self.turbulence(p, 7)).sin())
    }
}


#[cfg(test)]
mod test_texture{
    use super::*;

    #[test]
    fn test_solid_color_1() {
        let color = Color::new(1.0, 0.0, 0.0);
        let solid_color = SolidColor::new(color);
        let result = solid_color.value(0.0, 0.0, &Vec3d::zero());
        assert_eq!(result, color);
    }

    #[test]
    fn test_solid_color_2() {
        let color = Color::new(0.0, 1.0, 0.0);
        let solid_color = SolidColor::new(color);
        let result = solid_color.value(1.0, 0.7, &Vec3d::zero());
        assert_eq!(result, color);
    }

    #[test]
    fn test_checker_1() {
        let color1 = Color::new(1.0, 0.0, 0.0);
        let color2 = Color::new(0.0, 1.0, 0.0);
        let checker = Checker::from_color(color1, color2, 1.0);
        let result = checker.value(0.0, 0.0, &Vec3d::zero());
        assert_eq!(result, color1);

        let result = checker.value(0.0, 0.0, &Vec3d::new(1.0, 1.0, 1.0));
        assert_eq!(result, color2);
    }
}