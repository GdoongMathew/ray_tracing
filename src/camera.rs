use crate::vec3d::Vec3d;
use crate::object::{HittableVec, Hittable};
use crate::ray::{Ray, Interval};
use rand::Rng;
use crate::object::material::Scatterable;
use indicatif::ProgressBar;

pub struct Camera {
    center: Vec3d,
    focal_length: f64,
    aspect_ratio: f64,

    resolution: (i32, i32),
    viewport_dims: (f64, f64),

    viewport_u: Vec3d,
    viewport_v: Vec3d,

    samples_per_pixel: i32,
    samples_scale: f64,

    max_depth: i32,
}


impl Camera {
    pub fn new(
        center: Vec3d,
        focal_length: f64,
        aspect_ratio: f64,
        image_width: i32,
        viewport_height: f64,
    ) -> Self {
        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3d::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3d::new(0.0, -viewport_height, 0.0);

        let mut ret = Self {
            center,
            focal_length,
            aspect_ratio,
            resolution: (image_width, image_height),
            viewport_dims: (viewport_width, viewport_height),
            viewport_u,
            viewport_v,
            samples_per_pixel: 1,
            samples_scale: 1.0,
            max_depth: 10,
        };

        ret.set_samples_per_pixel(10);
        ret
    }
    pub fn set_center(&mut self, center: Vec3d) -> () { self.center = center; }

    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: i32) -> () {
        self.samples_per_pixel = samples_per_pixel;
        self.samples_scale = 1.0 / (samples_per_pixel as f64);
    }

    pub fn set_depth(&mut self, max_depth: i32) -> () { self.max_depth = max_depth; }

    pub fn resolution_width(&self) -> i32 { self.resolution.0 }

    pub fn resolution_height(&self) -> i32 { self.resolution.1 }

    pub fn viewport_width(&self) -> f64 { self.viewport_dims.0 }

    pub fn viewport_height(&self) -> f64 { self.viewport_dims.1 }

    pub fn pixel_delta_u(&self) -> Vec3d {
        // The pixel delta in the u (x) direction.
        self.viewport_u / self.resolution.0 as f64
    }

    pub fn pixel_delta_v(&self) -> Vec3d {
        // The pixel delta in the v (y) direction.
        self.viewport_v / self.resolution.1 as f64
    }

    pub fn viewport_upper_left(&self) -> Vec3d {
        self.center - self.viewport_u / 2.0 - self.viewport_v / 2.0 - Vec3d::new(0.0, 0.0, self.focal_length)
    }

    pub fn pixel_upper_left(&self) -> Vec3d {
        self.viewport_upper_left() + (self.pixel_delta_u() + self.pixel_delta_v()) * 0.5
    }

    /// Returns the center of the pixel at the given width and height coordinate.
    /// # Arguments
    /// * `w` - The width coordinate of the pixel.
    /// * `h` - The height coordinate of the pixel.
    pub fn pixel_coords(&self, w: f64, h: f64) -> Vec3d {
        self.pixel_upper_left() + self.pixel_delta_u() * w + self.pixel_delta_v() * h
    }

    fn ray_color<H: Hittable>(ray: &Ray, world: &H, depth: i32) -> Vec3d {
        if depth <= 0 { return Vec3d::zero(); }

        if let Some(hit_record) = world.hit(ray, &Interval { min: 0.0001, max: f64::INFINITY }) {
            let mut scatter = Ray::default();
            let mut attenuation = Vec3d::zero();

            return if hit_record.material.scatter(ray, &hit_record, &mut attenuation, &mut scatter) {
                attenuation * Self::ray_color(&scatter, world, depth - 1)
            } else {
                Vec3d::zero()
            };
        }

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3d::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3d::new(0.5, 0.7, 1.0) * a
    }

    /// Random sample a ray through the pixel at the given width and height coordinate.
    /// # Arguments
    /// * `i` - The width coordinate of the pixel.
    /// * `j` - The height coordinate of the pixel.
    fn sample_ray(&self, i: i32, j: i32) -> Ray {
        let mut rng = rand::thread_rng();

        let (offset_i, offset_j) = rng.random::<(f64, f64)>();

        let pixel_sample = self.pixel_coords(
            i as f64 + offset_i,
            j as f64 + offset_j,
        );

        let direction = pixel_sample - self.center;
        Ray::new(self.center, direction)
    }

    pub fn render(&self, world: &HittableVec) -> Vec<Vec3d> {
        let mut image = vec![
            Vec3d::new(0.0, 0.0, 0.0);
            (self.resolution_width() * self.resolution_height()) as usize
        ];

        let bar = ProgressBar::new(
            self.resolution_height() as u64 * self.resolution_width() as u64
        );

        for h in 0..self.resolution_height() {
            for w in 0..self.resolution_width() {
                let mut color = Vec3d::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.sample_ray(w, h);
                    color += Self::ray_color(&ray, world, self.max_depth);
                }
                image[(h * self.resolution_width() + w) as usize] = color * self.samples_scale;
                bar.inc(1);
            }
        }
        bar.finish_and_clear();
        image
    }
}

